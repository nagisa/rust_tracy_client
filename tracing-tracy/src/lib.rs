//! Collect [Tracy] profiles in tracing-enabled applications.
//!
//! Assuming the application is well instrumented, this should
//! in practice be a very low effort way to gain great amounts of insight into an application.
//!
//! [Tracy]: https://github.com/wolfpld/tracy

use std::fmt::Write;
use tracing_core::{
    field::{Field, Visit},
    span::Id,
    Event, Subscriber,
};
use tracing_subscriber::{
    layer::{Context, Layer},
    registry,
};

use tracy_client::{Span, message, finish_continuous_frame};

/// A tracing layer that collects data in Tracy profiling format.
#[derive(Clone)]
pub struct TracyLayer {
    stack_depth: u16,
}

impl TracyLayer {
    /// Create a new `TracyLayer`.
    ///
    /// Defaults to collecting stack traces.
    pub fn new() -> Self {
        Self { stack_depth: 64 }
    }

    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Specifying 0 frames will disable stack trace collection.
    pub fn with_stackdepth(mut self, stack_depth: u16) -> Self {
        self.stack_depth = stack_depth;
        self
    }
}

impl Default for TracyLayer {
    fn default() -> Self {
        Self::new()
    }
}

impl<S> Layer<S> for TracyLayer
where
    S: Subscriber + for<'a> registry::LookupSpan<'a>,
{
    fn on_enter(&self, id: &Id, ctx: Context<'_, S>) {
        if let Some(span_data) = ctx.span(id) {
            let metadata = span_data.metadata();
            // FIXME: look into adjusting tracy so allocating CStrings is not necessary.
            //
            // OTOH the fact that tracing reduces the lifetime of almost-always 'static data to
            // something less is also necessitating an allocation here...
            let file = metadata.file().unwrap_or("<error: not available>");
            let line = metadata.line().unwrap_or(0);
            let span = Span::new(metadata.name(), "", file, line, self.stack_depth);
            span_data.extensions_mut().insert(span);
        }
    }

    fn on_exit(&self, id: &Id, ctx: Context<'_, S>) {
        if let Some(span_data) = ctx.span(id) {
            if let Some(tracy_span) = span_data.extensions_mut().remove::<Span>() {
                drop(tracy_span);
            }
        }
    }

    fn on_event(&self, event: &Event, _: Context<'_, S>) {
        let mut visitor = TracyEventFieldVisitor {
            dest: String::new(),
            first: true,
            frame_mark: false,
        };
        event.record(&mut visitor);
        if !visitor.first {
            message(&visitor.dest, self.stack_depth)
        }
        if visitor.frame_mark {
            finish_continuous_frame!();
        }
    }
}

struct TracyEventFieldVisitor {
    dest: String,
    frame_mark: bool,
    first: bool,
}

impl Visit for TracyEventFieldVisitor {
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

    fn record_bool(&mut self, field: &Field, value: bool) {
        match (value, field.name()) {
            (true, "tracy_private.frame_mark") => self.frame_mark = true,
            _ => self.record_debug(field, &value),
        }
    }
}

#[cfg(test)]
mod tests {
    use tracing::{event, span, Level};
    use tracing_subscriber::layer::SubscriberExt;

    fn setup_subscriber() {
        static ONCE: std::sync::Once = std::sync::Once::new();
        ONCE.call_once(|| {
            tracing::subscriber::set_global_default(
                tracing_subscriber::registry().with(super::TracyLayer::new()),
            )
            .unwrap();
        });
    }

    #[test]
    fn it_works() {
        setup_subscriber();
        let span = span!(Level::TRACE, "a sec");
        let _enter = span.enter();
        event!(Level::INFO, "EXPLOSION!");
    }

    #[test]
    fn it_works_2() {
        setup_subscriber();
        let span = span!(Level::TRACE, "2 secs");
        let _enter = span.enter();
        event!(
            Level::INFO,
            message = "DOUBLE THE EXPLOSION!",
            tracy.frame_mark = true
        );
    }
}
