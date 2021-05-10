type __uint8_t = ::std::os::raw::c_uchar;
type __uint16_t = ::std::os::raw::c_ushort;
type __uint32_t = ::std::os::raw::c_uint;
type __uint64_t = ::std::os::raw::c_ulong;
extern "C" {
    pub fn ___tracy_set_thread_name(name: *const ::std::os::raw::c_char);
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_source_location_data {
    pub name: *const ::std::os::raw::c_char,
    pub function: *const ::std::os::raw::c_char,
    pub file: *const ::std::os::raw::c_char,
    pub line: u32,
    pub color: u32,
}
#[test]
fn bindgen_test_layout____tracy_source_location_data() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_source_location_data>(),
        32usize,
        concat!("Size of: ", stringify!(___tracy_source_location_data))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_source_location_data>(),
        8usize,
        concat!("Alignment of ", stringify!(___tracy_source_location_data))
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).name as *const _ as usize
        },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(name)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).function as *const _ as usize
        },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(function)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).file as *const _ as usize
        },
        16usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(file)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).line as *const _ as usize
        },
        24usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(line)
        )
    );
    assert_eq!(
        unsafe {
            &(*(::std::ptr::null::<___tracy_source_location_data>())).color as *const _ as usize
        },
        28usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_source_location_data),
            "::",
            stringify!(color)
        )
    );
}
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: ::std::os::raw::c_int,
}
#[test]
fn bindgen_test_layout____tracy_c_zone_context() {
    assert_eq!(
        ::std::mem::size_of::<___tracy_c_zone_context>(),
        8usize,
        concat!("Size of: ", stringify!(___tracy_c_zone_context))
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_c_zone_context>(),
        4usize,
        concat!("Alignment of ", stringify!(___tracy_c_zone_context))
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_c_zone_context>())).id as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_c_zone_context),
            "::",
            stringify!(id)
        )
    );
    assert_eq!(
        unsafe { &(*(::std::ptr::null::<___tracy_c_zone_context>())).active as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(___tracy_c_zone_context),
            "::",
            stringify!(active)
        )
    );
}
type TracyCZoneCtx = ___tracy_c_zone_context;
extern "C" {
    pub fn ___tracy_init_thread();
}
extern "C" {
    pub fn ___tracy_alloc_srcloc(
        line: u32,
        source: *const ::std::os::raw::c_char,
        sourceSz: usize,
        function: *const ::std::os::raw::c_char,
        functionSz: usize,
    ) -> u64;
}
extern "C" {
    pub fn ___tracy_alloc_srcloc_name(
        line: u32,
        source: *const ::std::os::raw::c_char,
        sourceSz: usize,
        function: *const ::std::os::raw::c_char,
        functionSz: usize,
        name: *const ::std::os::raw::c_char,
        nameSz: usize,
    ) -> u64;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin(
        srcloc: *const ___tracy_source_location_data,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_callstack(
        srcloc: *const ___tracy_source_location_data,
        depth: ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_alloc(
        srcloc: u64,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_begin_alloc_callstack(
        srcloc: u64,
        depth: ::std::os::raw::c_int,
        active: ::std::os::raw::c_int,
    ) -> TracyCZoneCtx;
}
extern "C" {
    pub fn ___tracy_emit_zone_end(ctx: TracyCZoneCtx);
}
extern "C" {
    pub fn ___tracy_emit_zone_text(
        ctx: TracyCZoneCtx,
        txt: *const ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    pub fn ___tracy_emit_zone_name(
        ctx: TracyCZoneCtx,
        txt: *const ::std::os::raw::c_char,
        size: usize,
    );
}
extern "C" {
    pub fn ___tracy_emit_zone_color(ctx: TracyCZoneCtx, color: u32);
}
extern "C" {
    pub fn ___tracy_emit_zone_value(ctx: TracyCZoneCtx, value: u64);
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc_callstack(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free(
        ptr: *const ::std::os::raw::c_void,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_callstack(
        ptr: *const ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_message(
        txt: *const ::std::os::raw::c_char,
        size: usize,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageL(
        txt: *const ::std::os::raw::c_char,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageC(
        txt: *const ::std::os::raw::c_char,
        size: usize,
        color: u32,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_messageLC(
        txt: *const ::std::os::raw::c_char,
        color: u32,
        callstack: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_frame_mark(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_mark_start(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_mark_end(name: *const ::std::os::raw::c_char);
}
extern "C" {
    pub fn ___tracy_emit_frame_image(
        image: *const ::std::os::raw::c_void,
        w: u16,
        h: u16,
        offset: u8,
        flip: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_emit_plot(name: *const ::std::os::raw::c_char, val: f64);
}
extern "C" {
    pub fn ___tracy_emit_message_appinfo(txt: *const ::std::os::raw::c_char, size: usize);
}
