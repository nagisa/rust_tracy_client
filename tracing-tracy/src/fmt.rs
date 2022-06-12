use tracing_core::field::Visit;
use tracing_core::Field;
use tracing_subscriber::field::{MakeVisitor, RecordFields, VisitFmt, VisitOutput};

/// Handle fields with the `tracy.` prefix specially.
///
/// * `tracy.frame_mark` event will cause a frame mark to be emitted;
/// * All other fields with the `tracy.` prefix are reserved and will be omitted.
pub struct TracyFields<F>(client::Client, F);

impl<F> TracyFields<F> {
    pub fn new(client: client::Client, inner: F) -> Self {
        Self(client, inner)
    }
}

impl<F, W> MakeVisitor<W> for TracyFields<F>
where
    F: MakeVisitor<W>,
{
    type Visitor = TracyFieldsVisitor<F::Visitor>;
    fn make_visitor(&self, target: W) -> Self::Visitor {
        TracyFieldsVisitor::new(self.0.clone(), self.1.make_visitor(target))
    }
}

pub struct TracyFieldsVisitor<V> {
    inner_visitor: V,
    client: client::Client,
    frame_mark: bool,
}

impl<V> TracyFieldsVisitor<V> {
    pub fn new(client: client::Client, inner: V) -> Self {
        Self {
            inner_visitor: inner,
            client,
            frame_mark: false,
        }
    }

    fn visit_field<Val, Cb: FnOnce(&mut V)>(&mut self, field: &Field, _: Val, otherwise: Cb) {
        if field.name().starts_with("tracy.") {
        } else {
            otherwise(&mut self.inner_visitor)
        }
    }
}

impl<V> Visit for TracyFieldsVisitor<V>
where
    V: Visit,
{
    fn record_debug(&mut self, field: &Field, value: &dyn std::fmt::Debug) {
        self.visit_field(field, value, |v| v.record_debug(field, value))
    }
    fn record_f64(&mut self, field: &Field, value: f64) {
        self.visit_field(field, value, |v| v.record_f64(field, value))
    }
    fn record_i64(&mut self, field: &Field, value: i64) {
        self.visit_field(field, value, |v| v.record_i64(field, value))
    }
    fn record_u64(&mut self, field: &Field, value: u64) {
        self.visit_field(field, value, |v| v.record_u64(field, value))
    }
    fn record_bool(&mut self, field: &Field, value: bool) {
        if value && field.name() == "tracy.frame_mark" {
            self.frame_mark = true;
        }
        self.visit_field(field, value, |v| v.record_bool(field, value))
    }
    fn record_str(&mut self, field: &Field, value: &str) {
        self.visit_field(field, value, |v| v.record_str(field, value))
    }
    fn record_error(&mut self, field: &Field, value: &(dyn std::error::Error + 'static)) {
        self.visit_field(field, value, |v| v.record_error(field, value))
    }
}

impl<V: VisitFmt> VisitFmt for TracyFieldsVisitor<V> {
    fn writer(&mut self) -> &mut dyn core::fmt::Write {
        self.inner_visitor.writer()
    }
}

impl<R, V: VisitOutput<R>> VisitOutput<R> for TracyFieldsVisitor<V> {
    fn finish(self) -> R {
        if self.frame_mark {
            self.client.frame_mark();
        }
        self.inner_visitor.finish()
    }

    fn visit<F: RecordFields>(self, fields: &F) -> R {
        self.inner_visitor.visit(fields)
    }
}

/// This formatter will not format any fields at all.
///
/// This will result in `tracy` falling back to showing function names for each span and event
/// isntead.
pub(crate) struct CollectMessageVisitor<'a>(pub(crate) Option<&'a str>);

impl<'a> Visit for CollectMessageVisitor<'a> {
    fn record_f64(&mut self, _: &Field, _: f64) {}
    fn record_i64(&mut self, _: &Field, _: i64) {}
    fn record_u64(&mut self, _: &Field, _: u64) {}
    fn record_bool(&mut self, _: &Field, _: bool) {}
    fn record_error(&mut self, _: &Field, _: &(dyn std::error::Error + 'static)) {}
    fn record_debug(&mut self, _: &Field, _: &dyn std::fmt::Debug) {}

    fn record_str(&mut self, field: &Field, value: &str) {
        if field.name() == "message" {
            unsafe {
                // SAFETY: this is somewhat tricky. We know this visitor will not outlive the event
                // being recorded, but there is no good way to tell rustc that because of
                // https://github.com/tokio-rs/valuable/issues/97
                self.0
                    .replace(std::str::from_utf8_unchecked(std::slice::from_raw_parts(
                        value.as_ptr(),
                        value.len(),
                    )));
            }
        }
    }
}

impl<'a> VisitOutput<Option<&'a str>> for CollectMessageVisitor<'a> {
    fn finish(self) -> Option<&'a str> {
        self.0
    }

    fn visit<F: RecordFields>(mut self, fields: &F) -> Option<&'a str> {
        fields.record(&mut self);
        self.finish()
    }
}
