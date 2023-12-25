//! Collect [Tracy] profiles in tracing-enabled applications.
//!
//! Assuming the application is well instrumented, this should in practice be a very low effort way
//! to gain great amounts of insight into an application performance.
//!
//! Note, however that Tracy is ultimately a profiling, not an observability, tool. As thus, some
//! of tracing concepts cannot be represented well by Tracy. For instance, out-of-order span
//! entries and exits, are not supported, and neither are spans that are entered and exited on
//! different threads. This crate will attempt to mitigate the problems and retain trace validity
//! at the cost of potentially invalid data. When such a mitigation occurs, trace will contain a
//! message with a note about the problem.
//!
//! Some other caveats to keep in mind:
//!
//! * Only span entries and exits are recorded;
//! * Events show up as messages in Tracy, however Tracy can struggle with large numbers of
//! messages;
//! * Some additional functionality such as plotting and memory allocation profiling is only
//! available as part of the [tracy-client](client) crate.
//!
//! # Examples
//!
//! The most basic way to setup the tracy subscriber globally is as follows:
//!
//! ```rust
//! use tracing_subscriber::layer::SubscriberExt;
//!
//! tracing::subscriber::set_global_default(
//!     tracing_subscriber::registry()
//!         .with(tracing_tracy::TracyLayer::new()),
//! ).expect("set up the subscriber");
//! ```
//!
//! # Important note
//!
//! Depending on the configuration Tracy may broadcast discovery packets to the local network and
//! expose the data it collects in the background to that same network. Traces collected by Tracy
//! may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracing-tracy` crate conditionally, via the
//! `enable` feature flag provided by this crate.
//!
//! [Tracy]: https://github.com/wolfpld/tracy
//!
//! # Features
//!
//! The following crate features are provided to customize the functionality of the Tracy client:
//!
#![doc = include_str!("../FEATURES.mkd")]
#![cfg_attr(tracing_tracy_docs, feature(doc_auto_cfg))]

use std::{borrow::Cow, cell::RefCell, collections::VecDeque, fmt::Write};
use tracing_core::{
    field::{Field, Visit},
    span::{Attributes, Id, Record},
    Event, Subscriber,
};
use tracing_subscriber::fmt::format::{DefaultFields, FormatFields};
use tracing_subscriber::{
    fmt::FormattedFields,
    layer::{Context, Layer},
    registry,
};

use client::{Client, Span};

pub use client;

thread_local! {
    /// A stack of spans currently active on the current thread.
    static TRACY_SPAN_STACK: RefCell<VecDeque<(Span, u64)>> =
        RefCell::new(VecDeque::with_capacity(16));
}

/// A tracing layer that collects data in Tracy profiling format.
#[derive(Clone)]
pub struct TracyLayer<F = DefaultFields> {
    fmt: F,
    stack_depth: u16,
    client: Client,
}

impl TracyLayer<DefaultFields> {
    /// Create a new `TracyLayer`.
    ///
    /// Defaults to collecting stack traces.
    #[must_use]
    pub fn new() -> Self {
        Self {
            fmt: DefaultFields::default(),
            stack_depth: 0,
            client: Client::start(),
        }
    }
}

impl<F> TracyLayer<F> {
    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Note that enabling callstack collection can and will introduce a non-trivial overhead at
    /// every instrumentation point. Specifying 0 frames (which is the default) will disable stack
    /// trace collection.
    #[must_use]
    pub const fn with_stackdepth(mut self, stack_depth: u16) -> Self {
        self.stack_depth = stack_depth;
        self
    }

    /// Use a custom field formatting implementation.
    #[must_use]
    pub fn with_formatter<Fmt>(self, fmt: Fmt) -> TracyLayer<Fmt> {
        TracyLayer {
            fmt,
            stack_depth: self.stack_depth,
            client: self.client,
        }
    }

    fn truncate_to_length<'d>(
        &self,
        data: &'d str,
        file: &str,
        function: &str,
        error_msg: &'static str,
    ) -> &'d str {
        // From AllocSourceLocation
        let mut max_len = usize::from(u16::MAX) - 2 - 4 - 4 - function.len() - 1 - file.len() - 1;
        if data.len() >= max_len {
            while !data.is_char_boundary(max_len) {
                max_len -= 1;
            }
            self.client
                .color_message(error_msg, 0xFF000000, self.stack_depth);
            &data[..max_len]
        } else {
            data
        }
    }
}

impl Default for TracyLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S, F> Layer<S> for TracyLayer<F>
where
    S: Subscriber + for<'a> registry::LookupSpan<'a>,
    F: for<'writer> FormatFields<'writer> + 'static,
{
    fn on_new_span(&self, attrs: &Attributes<'_>, id: &Id, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else { return };

        let mut extensions = span.extensions_mut();
        if extensions.get_mut::<FormattedFields<F>>().is_none() {
            let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
            if self.fmt.format_fields(fields.as_writer(), attrs).is_ok() {
                extensions.insert(fields);
            }
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        let Some(span) = ctx.span(id) else { return };

        let mut extensions = span.extensions_mut();
        if let Some(fields) = extensions.get_mut::<FormattedFields<F>>() {
            let _ = self.fmt.add_fields(fields, values);
        } else {
            let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
            if self.fmt.format_fields(fields.as_writer(), values).is_ok() {
                extensions.insert(fields);
            }
        }
    }

    fn on_event(&self, event: &Event, _: Context<'_, S>) {
        let mut visitor = TracyEventFieldVisitor {
            dest: String::with_capacity(64),
            first: true,
            frame_mark: false,
        };
        event.record(&mut visitor);
        if !visitor.first {
            self.client.message(
                self.truncate_to_length(
                    &visitor.dest,
                    "",
                    "",
                    "event message is too long and was truncated",
                ),
                self.stack_depth,
            );
        }
        if visitor.frame_mark {
            self.client.frame_mark();
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<S>) {
        let Some(span) = ctx.span(id) else { return };

        let metadata = span.metadata();
        let file = metadata.file().unwrap_or("<not available>");
        let line = metadata.line().unwrap_or(0);
        let name: Cow<str> = if let Some(fields) = span.extensions().get::<FormattedFields<F>>() {
            if fields.fields.is_empty() {
                metadata.name().into()
            } else {
                format!("{}{{{}}}", metadata.name(), fields.fields.as_str()).into()
            }
        } else {
            metadata.name().into()
        };
        TRACY_SPAN_STACK.with(|s| {
            s.borrow_mut().push_back((
                self.client.clone().span_alloc(
                    Some(self.truncate_to_length(
                        &name,
                        file,
                        "",
                        "span information is too long and was truncated",
                    )),
                    "",
                    file,
                    line,
                    self.stack_depth,
                ),
                id.into_u64(),
            ));
        });
    }

    fn on_exit(&self, id: &Id, _: Context<S>) {
        TRACY_SPAN_STACK.with(|s| {
            if let Some((span, span_id)) = s.borrow_mut().pop_back() {
                if id.into_u64() != span_id {
                    self.client.color_message(
                        "Tracing spans exited out of order! \
                        Trace may not be accurate for this span stack.",
                        0xFF000000,
                        self.stack_depth,
                    );
                }
                drop(span);
            } else {
                self.client.color_message(
                    "Exiting a tracing span, but got nothing on the tracy span stack!",
                    0xFF000000,
                    self.stack_depth,
                );
            }
        });
    }
}

struct TracyEventFieldVisitor {
    dest: String,
    frame_mark: bool,
    first: bool,
}

impl Visit for TracyEventFieldVisitor {
    fn record_bool(&mut self, field: &Field, value: bool) {
        match (value, field.name()) {
            (true, "tracy.frame_mark") => self.frame_mark = true,
            _ => self.record_debug(field, &value),
        }
    }

    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        // FIXME: this is a very crude formatter, but we don’t have
        // an easy way to do anything better...
        if self.first {
            let _ = write!(&mut self.dest, "{} = {:?}", field.name(), value);
            self.first = false;
        } else {
            let _ = write!(&mut self.dest, ", {} = {:?}", field.name(), value);
        }
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
fn main() {
    if std::env::args_os().any(|p| p == std::ffi::OsStr::new("--bench")) {
        tests::bench();
    } else {
        tests::test();
    }
}
