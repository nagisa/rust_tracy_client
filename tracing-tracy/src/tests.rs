use super::TracyLayer;
use futures::future::join_all;
use tracing::{debug, event, info, info_span, span, Level};
use tracing_attributes::instrument;
use tracing_subscriber::layer::SubscriberExt;

fn with_layer(f: impl FnOnce()) {
    let subscriber = tracing_subscriber::registry().with(TracyLayer::default());
    tracing::subscriber::with_default(subscriber, f);
}

#[test]
fn it_works() {
    with_layer(|| {
        let span = span!(Level::TRACE, "a sec");
        let _enter = span.enter();
        event!(Level::INFO, "EXPLOSION!");
    });
}

#[test]
fn it_works_2() {
    with_layer(|| {
        let span = span!(Level::TRACE, "2 secs");
        let _enter = span.enter();
        event!(
            Level::INFO,
            message = "DOUBLE THE EXPLOSION!",
            tracy.frame_mark = true
        );
    });
}

#[test]
fn multiple_entries() {
    with_layer(|| {
        let span = span!(Level::INFO, "multiple_entries");
        span.in_scope(|| {});
        span.in_scope(|| {});

        let span = span!(Level::INFO, "multiple_entries 2");
        span.in_scope(|| span.in_scope(|| {}));
    });
}

#[test]
fn out_of_order() {
    with_layer(|| {
        let span1 = span!(Level::INFO, "out of order exits 1");
        let span2 = span!(Level::INFO, "out of order exits 2");
        let span3 = span!(Level::INFO, "out of order exits 3");
        let entry1 = span1.enter();
        let entry2 = span2.enter();
        let entry3 = span3.enter();
        drop(entry2);
        drop(entry3);
        drop(entry1);
    });
}

#[test]
fn exit_in_different_thread() {
    with_layer(|| {
        let span = Box::new(span!(Level::INFO, "exit in different thread"));
        let entry = span.enter();
        std::thread::scope(|scope| {
            let thread = scope.spawn(move || drop(entry));
            thread.join().unwrap();
        });
    });
}

async fn parent_task(subtasks: usize) {
    info!("spawning subtasks...");
    let subtasks = (1..=subtasks)
        .map(|number| {
            debug!(message = "creating subtask;", number);
            subtask(number)
        })
        .collect::<Vec<_>>();

    let result = join_all(subtasks).await;

    debug!("all subtasks completed");
    let sum: usize = result.into_iter().sum();
    info!(sum);
}

#[instrument]
async fn subtask(number: usize) -> usize {
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(number as _)).await;
    info!("sleeping in subtask {}...", number);
    tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    number
}

// Test based on the spawny_things example from the tracing repository.
#[test]
fn async_futures() {
    with_layer(|| {
        let runtime = tokio::runtime::Builder::new_current_thread()
            .enable_all()
            .build()
            .expect("tokio runtime");
        runtime.block_on(parent_task(5));
    });
}

#[test]
fn message_too_long() {
    with_layer(|| {
        info!("{}", "a".repeat(u16::MAX.into()));
    });
}

#[test]
fn long_span_data() {
    with_layer(|| {
        let data = "c".repeat(u16::MAX.into());
        info_span!("some span name", "{}", data).in_scope(|| {});
    });
}

#[test]
fn span_with_fields() {
    with_layer(|| {
        let span = span!(
            Level::TRACE,
            "wait",
            duration = 0,
            reason = "testing fields"
        );
        let _enter = span.enter();
    });
}
