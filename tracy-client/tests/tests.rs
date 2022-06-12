use tracy_client::*;

#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

fn basic_zone() {
    let client = Client::start();
    let span = client.span(span_location!("basic_zone"), 100);
    span.emit_value(42);
    span.emit_text("some text");
    for i in 322..420 {
        span.emit_value(i);
    }
}

fn alloc_zone() {
    let client = Client::start();
    let span = client.span_alloc(Some("alloc_zone"), "alloc_zone", file!(), line!(), 100);
    span.emit_value(42);
    span.emit_color(0x00FF0000);
    span.emit_text("some text");
}

fn finish_frameset() {
    let client = Client::start();
    for _ in 0..10 {
        client.frame_mark();
    }
    frame_mark();
}

fn finish_secondary_frameset() {
    let client = Client::start();
    for _ in 0..5 {
        client.secondary_frame_mark(frame_name!("secondary frame"));
    }
    secondary_frame_mark!("secondary frame macro");
}

fn non_continuous_frameset() {
    const NON_CONTINUOUS: FrameName = frame_name!("non continuous");
    let client = Client::start();
    client.non_continuous_frame(NON_CONTINUOUS);
    non_continuous_frame!("non continuous macro");
}

fn plot_something() {
    static TEMPERATURE: PlotName = plot_name!("temperature");
    let client = Client::start();
    for i in 0..10 {
        client.plot(TEMPERATURE, i as f64);
    }

    plot!("temperature", 42.0);
}

fn allocations() {
    let mut strings = Vec::new();
    for i in 0..100 {
        strings.push(format!("{:?}", i));
    }
}

fn fib(i: u16) -> u64 {
    let span = span!();
    span.emit_text(&format!("fib({})", i));
    let result = match i {
        0 => 0,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    };
    span.emit_value(result);
    result
}

fn message() {
    let client = Client::start();
    client.message("test message", 100);
    client.message("test message without stack", 0);
}

fn tls_confusion() {
    let client = Client::start();
    let t1 = std::thread::spawn(move || {
        drop(client);
    });
    let _ = t1.join();
    let _ = Client::start();
}

fn set_thread_name() {
    let _client = Client::start();
    set_thread_name!("test thread");
}

fn nameless_span() {
    let client = Client::start();
    span!();
    client.span_alloc(None, "nameless_span", file!(), line!(), 0);
    set_thread_name!("test thread");
}

fn main() {
    #[cfg(not(loom))]
    {
        basic_zone();
        alloc_zone();
        finish_frameset();
        finish_secondary_frameset();
        non_continuous_frameset();
        plot_something();
        message();
        allocations();
        tls_confusion();
        nameless_span();
        let thread = std::thread::spawn(|| {
            let _client = Client::start();
            fib(20);
        });
        thread.join().unwrap();
        set_thread_name();
    }
}
