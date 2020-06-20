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

extern "C" {
    pub fn ___tracy_init_thread();
    pub fn ___tracy_alloc_srcloc(
        line: u32,
        source: *const c_char,
        source_len: usize,
        function: *const c_char,
        function_len: usize
    ) -> u64;
    pub fn ___tracy_alloc_srcloc_name(
        line: u32,
        source: *const c_char,
        source_len: usize,
        function: *const c_char,
        function_len: usize,
        name: *const c_char,
        name_len: usize
    ) -> u64;
    pub fn ___tracy_emit_zone_begin(
        srcloc: *const ___tracy_source_location_data,
        active: c_int
    ) -> TracyCZoneCtx;
    pub fn ___tracy_emit_zone_begin_callstack(
        srcloc: *const ___tracy_source_location_data,
        depth: c_int,
        active: c_int
    ) -> TracyCZoneCtx;
    pub fn ___tracy_emit_zone_begin_alloc(
        srcloc: u64,
        active: c_int
    ) -> TracyCZoneCtx;
    pub fn ___tracy_emit_zone_begin_alloc_callstack(
        srcloc: u64,
        depth: c_int,
        active: c_int
    ) -> TracyCZoneCtx;

    pub fn ___tracy_emit_zone_end(ctx: TracyCZoneCtx);
    pub fn ___tracy_emit_zone_text(ctx: TracyCZoneCtx, txt: *const c_char, size: usize);
    pub fn ___tracy_emit_zone_name(ctx: TracyCZoneCtx, txt: *const c_char, size: usize);
    pub fn ___tracy_emit_zone_value(ctx: TracyCZoneCtx, value: u64);
    pub fn ___tracy_emit_memory_alloc(ptr: *const c_void, size: usize);
    pub fn ___tracy_emit_memory_alloc_callstack(ptr: *const c_void, size: usize, depth: c_int);
    pub fn ___tracy_emit_memory_free(ptr: *const c_void);
    pub fn ___tracy_emit_memory_free_callstack(ptr: *const c_void, depth: c_int);
    pub fn ___tracy_emit_message( txt: *const c_char, size: usize, callstack: c_int);
    pub fn ___tracy_emit_messageL( txt: *const c_char, callstack: c_int);
    pub fn ___tracy_emit_messageC( txt: *const c_char, size: usize, color: u32, callstack: c_int);
    pub fn ___tracy_emit_messageLC( txt: *const c_char, color: u32, callstack: c_int);
    pub fn ___tracy_emit_frame_mark(name: *const c_char);
    pub fn ___tracy_emit_frame_mark_start(name: *const c_char);
    pub fn ___tracy_emit_frame_mark_end(name: *const c_char);
    pub fn ___tracy_emit_frame_image(image: *const c_void, w: u16, h: u16, offset: u8, flip: c_int);
    pub fn ___tracy_emit_plot(name: *const c_char, val: f64);
    pub fn ___tracy_emit_message_appinfo(txt: *const c_char, size: usize);
}

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
