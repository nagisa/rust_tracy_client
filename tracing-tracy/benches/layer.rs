use criterion::Criterion;
use tracing_subscriber::layer::SubscriberExt;
use tracing_tracy::{Config, DefaultConfig, TracyLayer};

#[derive(Default)]
struct CallstackConfig(DefaultConfig);
impl Config for CallstackConfig {
    type Formatter = <DefaultConfig as Config>::Formatter;
    fn formatter(&self) -> &Self::Formatter {
        self.0.formatter()
    }
    fn stack_depth(&self, _: &tracing_core::Metadata<'_>) -> u16 {
        100
    }
}

fn benchmark_span(c: &mut Criterion) {
    c.bench_function("span/callstack", |bencher| {
        let layer =
            tracing_subscriber::registry().with(TracyLayer::new(CallstackConfig::default()));
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                let _span =
                    tracing::error_span!("message", field1 = "first", field2 = "second").entered();
            });
        });
    });

    c.bench_function("span/no_callstack", |bencher| {
        let layer = tracing_subscriber::registry().with(TracyLayer::default());
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                let _span =
                    tracing::error_span!("message", field1 = "first", field2 = "second").entered();
            });
        });
    });
}

fn benchmark_message(c: &mut Criterion) {
    c.bench_function("event/callstack", |bencher| {
        let layer =
            tracing_subscriber::registry().with(TracyLayer::new(CallstackConfig::default()));
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                tracing::error!(field1 = "first", field2 = "second", "message");
            });
        });
    });

    c.bench_function("event/no_callstack", |bencher| {
        let layer = tracing_subscriber::registry().with(TracyLayer::default());
        tracing::subscriber::with_default(layer, || {
            bencher.iter(|| {
                tracing::error!(field1 = "first", field2 = "second", "message");
            });
        });
    });
}

criterion::criterion_group!(benches, benchmark_span, benchmark_message);
criterion::criterion_main!(benches);
