#![cfg(feature = "enable")]

use tracy_client_sys::*;

fn with_profiler(f: impl FnOnce()) {
    #[cfg(feature = "manual-lifetime")]
    unsafe {
        ___tracy_startup_profiler();
    }
    f();
    #[cfg(feature = "manual-lifetime")]
    unsafe {
        ___tracy_shutdown_profiler();
    }
}

#[test]
fn emit_zone() {
    with_profiler(|| unsafe {
        let srcloc = Box::leak(Box::new(___tracy_source_location_data {
            name: b"name\0".as_ptr().cast(),
            function: b"function\0".as_ptr().cast(),
            file: b"file\0".as_ptr().cast(),
            line: 42,
            color: 0,
        }));
        let zone_ctx = ___tracy_emit_zone_begin(srcloc, 1);
        ___tracy_emit_zone_end(zone_ctx);
    });
}

#[test]
fn emit_message_no_null() {
    with_profiler(|| unsafe {
        ___tracy_emit_logString(
            TracyMessageSeverity_TracyMessageSeverityInfo as i8,
            0,
            1,
            11,
            b"hello world".as_ptr().cast(),
        );
    });
}

#[cfg(feature = "fibers")]
#[test]
fn fiber_enter_leave() {
    with_profiler(|| unsafe {
        ___tracy_fiber_enter(b"hello\0".as_ptr().cast());
        ___tracy_fiber_leave();
    });
}
