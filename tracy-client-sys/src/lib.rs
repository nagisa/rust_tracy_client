//! Low level API to the Tracy Client
//!
//! This crate builds the C++ tracy client and has no external dependencies.
//!
//! For a higher-level API consider `tracy-client`.
//!
//! # Important note
//!
//! Simply depending on this crate is sufficient for tracy to be enabled at program startup, even
//! if none of the APIs provided by this crate are invoked. Tracy will broadcast discovery packets
//! to the local network and expose the data it collects in the background to that same network.
//! Traces collected by Tracy may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client-sys` crate conditionally, via
//! the `enable` feature flag provided by this crate.

use std::os::raw::{c_char, c_int};
use std::ffi::c_void;

#[repr(C)]
#[derive(Copy, Clone)]
pub struct ___tracy_source_location_data {
    pub name: *const c_char,
    pub function: *const c_char,
    pub file: *const c_char,
    pub line: u32,
    pub color: u32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct TracyCZoneCtx {
    pub id: u32,
    pub active: c_int,
}

#[cfg(feature="enable")]
macro_rules! enabled_fn {
    (pub $($tokens: tt)*) => {
        extern "C" { pub $($tokens)*; }
    };
}

#[cfg(not(feature="enable"))]
macro_rules! enabled_fn {
    (pub $($tokens: tt)*) => {
        #[allow(non_snake_case, unused_variables)]
        #[inline(always)]
        pub unsafe extern "C" $($tokens)* { std::mem::zeroed() }
    };
}

enabled_fn! { pub fn ___tracy_init_thread() }
enabled_fn! { pub fn ___tracy_alloc_srcloc(
    line: u32,
    source: *const c_char,
    source_len: usize,
    function: *const c_char,
    function_len: usize
) -> u64 }
enabled_fn! { pub fn ___tracy_alloc_srcloc_name(
    line: u32,
    source: *const c_char,
    source_len: usize,
    function: *const c_char,
    function_len: usize,
    name: *const c_char,
    name_len: usize
) -> u64 }
enabled_fn! { pub fn ___tracy_emit_zone_begin(
    srcloc: *const ___tracy_source_location_data,
    active: c_int
) -> TracyCZoneCtx }
enabled_fn! { pub fn ___tracy_emit_zone_begin_callstack(
    srcloc: *const ___tracy_source_location_data,
    depth: c_int,
    active: c_int
) -> TracyCZoneCtx }
enabled_fn! { pub fn ___tracy_emit_zone_begin_alloc(
    srcloc: u64,
    active: c_int
) -> TracyCZoneCtx }
enabled_fn! { pub fn ___tracy_emit_zone_begin_alloc_callstack(
    srcloc: u64,
    depth: c_int,
    active: c_int
) -> TracyCZoneCtx }
enabled_fn! { pub fn ___tracy_emit_zone_end(
    ctx: TracyCZoneCtx
) }
enabled_fn! { pub fn ___tracy_emit_zone_text(
    ctx: TracyCZoneCtx,
    txt: *const c_char,
    size: usize
) }
enabled_fn! { pub fn ___tracy_emit_zone_name(
    ctx: TracyCZoneCtx,
    txt: *const c_char,
    size: usize
) }
enabled_fn! { pub fn ___tracy_emit_zone_value(
    ctx: TracyCZoneCtx,
    value: u64
) }
enabled_fn! { pub fn ___tracy_emit_memory_alloc(
    ptr: *const c_void,
    size: usize,
    secure: c_int
) }
enabled_fn! { pub fn ___tracy_emit_memory_alloc_callstack(
    ptr: *const c_void,
    size: usize,
    depth: c_int,
    secure: c_int
) }
enabled_fn! { pub fn ___tracy_emit_memory_free(
    ptr: *const c_void,
    secure: c_int
) }
enabled_fn! { pub fn ___tracy_emit_memory_free_callstack(
    ptr: *const c_void,
    depth: c_int,
    secure: c_int
) }
enabled_fn! { pub fn ___tracy_emit_message(
    txt: *const c_char,
    size: usize,
    callstack: c_int
) }
enabled_fn! { pub fn ___tracy_emit_messageL(
    txt: *const c_char,
    callstack: c_int
) }
enabled_fn! { pub fn ___tracy_emit_messageC(
    txt: *const c_char,
    size: usize, color: u32,
    callstack: c_int
) }
enabled_fn! { pub fn ___tracy_emit_messageLC(
    txt: *const c_char,
    color: u32,
    callstack: c_int) }
enabled_fn! { pub fn ___tracy_emit_frame_mark(
    name: *const c_char
) }
enabled_fn! { pub fn ___tracy_emit_frame_mark_start(
    name: *const c_char
) }
enabled_fn! { pub fn ___tracy_emit_frame_mark_end(
    name: *const c_char
) }
enabled_fn! { pub fn ___tracy_emit_frame_image(
    image: *const c_void,
    w: u16,
    h: u16,
    offset: u8,
    flip: c_int
) }
enabled_fn! { pub fn ___tracy_emit_plot(
    name: *const c_char,
    val: f64
) }
enabled_fn! { pub fn ___tracy_emit_message_appinfo(
    txt: *const c_char,
    size: usize
) }

enabled_fn! { pub fn ___tracy_set_thread_name(
    name: *const c_char,
) }

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emit_zone() {
        unsafe {
            let srcloc = ___tracy_source_location_data {
                name: b"name\0".as_ptr() as _,
                function: b"function\0".as_ptr() as _,
                file: b"file\0".as_ptr() as _,
                line: 42,
                color: 0,
            };
            let zone_ctx = ___tracy_emit_zone_begin(&srcloc, 1);
            ___tracy_emit_zone_end(zone_ctx);
        }
    }

    #[test]
    fn emit_message_no_null() {
        unsafe {
            ___tracy_emit_message(b"hello world".as_ptr() as _, 11, 1);
        }
    }
}
