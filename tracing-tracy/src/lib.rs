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
//! Refer to the [`client::sys`] crate for documentation on crate features. This crate re-exports
//! all the features from [`client`].

use buffers::BufferPool;
use client::{Client, Span};
use std::cell::RefCell;
use std::collections::VecDeque;
use tracing_core::span::{Attributes, Id, Record};
use tracing_core::{Event, Subscriber};
use tracing_subscriber::fmt::format::{DefaultFields, FormatFields};
use tracing_subscriber::fmt::FormattedFields;
use tracing_subscriber::layer::{Context, Layer};
use tracing_subscriber::registry;

pub use client;
mod buffers;
pub mod fmt;

const DEFAULT_BUFFER_CACHE_SIZE: usize = 32;
const DEFAULT_BUFFER_SIZE: usize = 64;

thread_local! {
    /// A stack of spans currently active on the current thread.
    static TRACY_SPAN_STACK: RefCell<VecDeque<(Span, u64)>> =
        RefCell::new(VecDeque::with_capacity(16));
}

/// A tracing layer that collects data in Tracy profiling format.
#[derive(Clone)]
pub struct TracyLayer<F = fmt::TracyFields<DefaultFields>> {
    fmt: F,
    client: Client,
    buffer_pool: BufferPool<F>,

    stack_depth: u16,
    key_values: bool,
}

impl TracyLayer<fmt::TracyFields<DefaultFields>> {
    /// Create a new `TracyLayer`.
    ///
    /// Defaults to collecting stack traces.
    pub fn new() -> Self {
        let client = Client::start();
        Self {
            fmt: fmt::TracyFields::new(client.clone(), DefaultFields::default()),
            client,
            buffer_pool: BufferPool::new(DEFAULT_BUFFER_CACHE_SIZE, DEFAULT_BUFFER_SIZE),

            stack_depth: 0,
            key_values: true,
        }
    }
}

impl<F> TracyLayer<F> {
    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Note that enabling callstack collection can and will introduce a non-trivial overhead at
    /// every instrumentation point. Specifying 0 frames (which is the default) will disable stack
    /// trace collection.
    pub fn with_stackdepth(mut self, stack_depth: u16) -> Self {
        self.stack_depth = stack_depth;
        self
    }

    /// Use a custom field formatting implementation for span names and events.
    pub fn with_formatter<Fmt>(self, fmt: Fmt) -> TracyLayer<Fmt> {
        let num_buffers = self.buffer_pool.pool.capacity();
        let buffer_size = self.buffer_pool.buffer_size;
        TracyLayer {
            fmt,
            client: self.client,
            buffer_pool: self.buffer_pool.remake(num_buffers, buffer_size),
            key_values: self.key_values,
            stack_depth: self.stack_depth,
        }
    }

    /// Preallocate a formatting buffer cache with the specified number of elements.
    ///
    /// By default the buffer cache holds 32 items.
    pub fn with_buffer_cache(mut self, num_buffers: usize, buffer_size: usize) -> Self {
        self.buffer_pool = self.buffer_pool.remake(num_buffers, buffer_size);
        self
    }

    /// Whether to emit key-values as span names and messages to Tracy.
    ///
    /// Formatting key-values will prominently display them in the trace output for each span and
    /// event. Disabling this can significantly reduce the overhead of instrumentation. When
    /// doing so the spans in traces will fall back to displaying only the span name (often the
    /// same as the instrumented function’s name), while events will only display the contents of
    /// the message key. The rest of the information is discarded.
    pub fn with_keys_and_values(mut self, enable: bool) -> Self {
        self.key_values = enable;
        self
    }

    fn truncate_to_length<'d>(
        &self,
        data: &'d str,
        file: &str,
        function: &str,
        error_msg: &'static str,
    ) -> &'d str {
        // From AllocSourceLocation
        let mut max_len =
            usize::from(u16::max_value()) - 2 - 4 - 4 - function.len() - 1 - file.len() - 1;
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
        if self.key_values {
            if let Some(span) = ctx.span(id) {
                let mut fields = self.buffer_pool.get();
                if self.fmt.format_fields(fields.as_writer(), attrs).is_ok() {
                    span.extensions_mut().insert(fields);
                }
            }
        }
    }

    fn on_record(&self, id: &Id, values: &Record<'_>, ctx: Context<'_, S>) {
        if self.key_values {
            if let Some(span) = ctx.span(id) {
                let mut extensions = span.extensions_mut();
                if let Some(fields) = extensions.get_mut::<buffers::Buffer<F>>() {
                    let _ = self.fmt.add_fields(fields, values);
                } else {
                    let mut fields = FormattedFields::<F>::new(String::with_capacity(64));
                    if self.fmt.format_fields(fields.as_writer(), values).is_ok() {
                        extensions.insert(fields);
                    }
                }
            }
        }
    }

    fn on_enter(&self, id: &Id, ctx: Context<S>) {
        if let Some(span_data) = ctx.span(id) {
            let metadata = span_data.metadata();
            let file = metadata.file().unwrap_or("<not available>");
            let line = metadata.line().unwrap_or(0);
            let extensions;
            let key_values: Option<&str> = if self.key_values {
                extensions = span_data.extensions();
                if let Some(fields) = extensions.get::<buffers::Buffer<F>>() {
                    if fields.fields.as_str().is_empty() {
                        None
                    } else {
                        Some(fields.fields.as_str())
                    }
                } else {
                    None
                }
            } else {
                None
            };
            TRACY_SPAN_STACK.with(|s| {
                let span = self.client.clone().span_alloc(
                    None,
                    metadata.name(),
                    file,
                    line,
                    self.stack_depth,
                );
                if let Some(key_values) = key_values {
                    // TODO: maybe emit each KV as a separate text?
                    span.emit_text(self.truncate_to_length(
                        key_values,
                        "",
                        "",
                        "key-values were truncated",
                    ));
                }
                s.borrow_mut().push_back((span, id.into_u64()));
            });
        }
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

    fn on_event(&self, event: &Event, _: Context<'_, S>) {
        if self.key_values {
            buffers::with_event_buffer::<F, _>(|ff| {
                self.fmt.format_fields(ff.as_writer(), event).unwrap();
                self.client.message(
                    self.truncate_to_length(
                        ff.as_str(),
                        "",
                        "",
                        "event message is too long and was truncated",
                    ),
                    self.stack_depth,
                );
            });
        } else {
            let message_collector =
                fmt::TracyFieldsVisitor::new(self.client.clone(), fmt::CollectMessageVisitor(None));
            let message = tracing_subscriber::field::VisitOutput::visit(message_collector, &event);
            if let Some(msg) = message {
                self.client.message(
                    self.truncate_to_length(
                        msg,
                        "",
                        "",
                        "event message is too long and was truncated",
                    ),
                    self.stack_depth,
                );
            } else {
                self.client
                    .color_message("event without message", 0xFF000000, self.stack_depth);
            }
        }
    }
}

#[cfg(test)]
mod tests;
#[cfg(test)]
fn main() {
    tests::main();
}
