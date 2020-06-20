use tracy_client::*;
use std::thread::{spawn, sleep};

#[global_allocator]
static GLOBAL: ProfiledAllocator<std::alloc::System> =
    ProfiledAllocator::new(std::alloc::System, 100);

fn fib(i: u16) -> u64 {
    let span = Span::new(&format!("fib({})", i), "fib", file!(), line!(), 100);
    let result = match i {
        0 => 0,
        1 => 1,
        _ => fib(i - 1) + fib(i - 2),
    };
    span.emit_value(result);
    result
}

fn main() {
    message("starting T1", 10);
    let t1 = spawn(|| {
        for _ in 0..100 {
            let span = Span::new("zone values", "zone_values", file!(), line!(), 100);
            span.emit_value(42);
            sleep(std::time::Duration::from_secs(1));
            span.emit_value(322);
            sleep(std::time::Duration::from_secs(1));
            span.emit_value(101);
            sleep(std::time::Duration::from_secs(1));
            span.emit_value(101 - 5);
            finish_continuous_frame!("T1");
        }
    });

    message("starting T2", 10);
    let t2 = spawn(|| {
        for _ in 0..100 {
            let span = Span::new("zone text", "zone_text", file!(), line!(), 100);
            span.emit_text("sleeping first time");
            std::thread::sleep(std::time::Duration::from_secs(1));
            let string = format!("sleeping second time");
            span.emit_text(&string);
            drop(string);
            std::thread::sleep(std::time::Duration::from_secs(1));
            let string = format!("sleeping third time");
            span.emit_text(&string);
            drop(string);
            std::thread::sleep(std::time::Duration::from_secs(1));
            finish_continuous_frame!("T2");
        }
    });

    message("starting t3", 10);
    let t3 = spawn(|| {
        for _ in 0..100 {
            std::thread::sleep(std::time::Duration::from_secs(1));
            finish_continuous_frame!();
            finish_continuous_frame!("T3")
        }
    });

    message("starting t4", 10);
    let t4 = spawn(|| {
        static PLOT: Plot = create_plot!("random numbers");
        let mut seed = 42u32;
        for _ in 0..100 {
            seed = (seed * 1103515245 + 12345) & 0x7fffffff;
            PLOT.point(seed as f64);
            std::thread::sleep(std::time::Duration::from_secs(1));
            finish_continuous_frame!("T4")
        }
    });

    message("starting t5", 10);
    let t5 = spawn(|| {
        for i in 0..100 {
            {
                let _f = start_noncontinuous_frame!("making vectors");
                message(&format!("making vector of {} vectors", i), 20);
                let mut vec = Vec::new();
                for v in (0..i).map(|v| Vec::<u8>::with_capacity(v * 100)) {
                    vec.push(v)
                }
            }
            std::thread::sleep(std::time::Duration::from_secs(1));
            finish_continuous_frame!("T5");
        }
    });


    message("starting t6", 10);
    let t6 = spawn(|| {
        fib(25);
    });

    let _ = t1.join();
    message("T1 joined", 10);
    let _ = t2.join();
    message("T2 joined", 10);
    let _ = t3.join();
    message("T3 joined", 10);
    let _ = t4.join();
    message("T4 joined", 10);
    let _ = t5.join();
    message("T5 joined", 10);
    let _ = t6.join();
    message("T6 joined", 10);
}
