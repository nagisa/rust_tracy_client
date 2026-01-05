use std::time::Duration;
use std::{ffi::CString, thread};
use tracy_client::{span, Client};
use tracy_client_sys::{
    ___tracy_emit_zone_begin, ___tracy_emit_zone_end, ___tracy_fiber_enter, ___tracy_fiber_leave,
};

pub fn main() {
    Client::start();

    let mut zone = tracy_client_sys::___tracy_c_zone_context { id: 0, active: 0 };
    let fiber = CString::new("job1").unwrap();

    thread::scope(|s| {
        println!("t1");

        s.spawn(|| {
            println!("t1 fiber enter");
            unsafe { ___tracy_fiber_enter(fiber.as_ptr()) };
            println!("t1 begin alloc");
            let location: &'static tracy_client::SpanLocation =
                tracy_client::span_location!("hi hi");
            let ctx = unsafe { ___tracy_emit_zone_begin(&location.data, 1) };
            zone = ctx;
            println!("t1 sleep");
            let t1_span = span!("t1 span");
            thread::sleep(Duration::from_secs(1));
            drop(t1_span);
            println!("t1 fiber leave");
            unsafe { ___tracy_fiber_leave() };
        });

        println!("t1 joined");
    });

    thread::scope(|s| {
        println!("t2");

        s.spawn(|| {
            println!("t2 fiber enter");
            unsafe { ___tracy_fiber_enter(fiber.as_ptr()) };
            println!("t2 sleep");
            let t2_span = span!("t2 span");
            thread::sleep(Duration::from_millis(100));

            let span = span!("t2 subspan");
            thread::sleep(Duration::from_millis(100));
            drop(span);

            thread::sleep(Duration::from_millis(100));

            let span = span!("t2 subspan");
            thread::sleep(Duration::from_millis(100));
            drop(span);

            thread::sleep(Duration::from_secs(1));
            println!("t2 zone end");
            unsafe { ___tracy_emit_zone_end(zone) };
            println!("t2 fiber leave");
            drop(t2_span);
            unsafe { ___tracy_fiber_leave() };
        });

        println!("t2 joined");
    });
}
