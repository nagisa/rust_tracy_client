pub const TracyPlotFormatEnum_TracyPlotFormatNumber: TracyPlotFormatEnum = 0;
pub const TracyPlotFormatEnum_TracyPlotFormatMemory: TracyPlotFormatEnum = 1;
pub const TracyPlotFormatEnum_TracyPlotFormatPercentage: TracyPlotFormatEnum = 2;
pub const TracyPlotFormatEnum_TracyPlotFormatWatt: TracyPlotFormatEnum = 3;
type TracyPlotFormatEnum = ::std::os::raw::c_uint;
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
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_source_location_data"]
        [::std::mem::size_of::<___tracy_source_location_data>() - 32usize];
    ["Alignment of ___tracy_source_location_data"]
        [::std::mem::align_of::<___tracy_source_location_data>() - 8usize];
    ["Offset of field: ___tracy_source_location_data::name"]
        [::std::mem::offset_of!(___tracy_source_location_data, name) - 0usize];
    ["Offset of field: ___tracy_source_location_data::function"]
        [::std::mem::offset_of!(___tracy_source_location_data, function) - 8usize];
    ["Offset of field: ___tracy_source_location_data::file"]
        [::std::mem::offset_of!(___tracy_source_location_data, file) - 16usize];
    ["Offset of field: ___tracy_source_location_data::line"]
        [::std::mem::offset_of!(___tracy_source_location_data, line) - 24usize];
    ["Offset of field: ___tracy_source_location_data::color"]
        [::std::mem::offset_of!(___tracy_source_location_data, color) - 28usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: ::std::os::raw::c_int,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_c_zone_context"][::std::mem::size_of::<___tracy_c_zone_context>() - 8usize];
    ["Alignment of ___tracy_c_zone_context"]
        [::std::mem::align_of::<___tracy_c_zone_context>() - 4usize];
    ["Offset of field: ___tracy_c_zone_context::id"]
        [::std::mem::offset_of!(___tracy_c_zone_context, id) - 0usize];
    ["Offset of field: ___tracy_c_zone_context::active"]
        [::std::mem::offset_of!(___tracy_c_zone_context, active) - 4usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_time_data {
    pub gpuTime: i64,
    pub queryId: u16,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_time_data"][::std::mem::size_of::<___tracy_gpu_time_data>() - 16usize];
    ["Alignment of ___tracy_gpu_time_data"]
        [::std::mem::align_of::<___tracy_gpu_time_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_time_data::gpuTime"]
        [::std::mem::offset_of!(___tracy_gpu_time_data, gpuTime) - 0usize];
    ["Offset of field: ___tracy_gpu_time_data::queryId"]
        [::std::mem::offset_of!(___tracy_gpu_time_data, queryId) - 8usize];
    ["Offset of field: ___tracy_gpu_time_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_time_data, context) - 10usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_zone_begin_data {
    pub srcloc: u64,
    pub queryId: u16,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_zone_begin_data"]
        [::std::mem::size_of::<___tracy_gpu_zone_begin_data>() - 16usize];
    ["Alignment of ___tracy_gpu_zone_begin_data"]
        [::std::mem::align_of::<___tracy_gpu_zone_begin_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_zone_begin_data::srcloc"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_data, srcloc) - 0usize];
    ["Offset of field: ___tracy_gpu_zone_begin_data::queryId"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_data, queryId) - 8usize];
    ["Offset of field: ___tracy_gpu_zone_begin_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_data, context) - 10usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_zone_begin_callstack_data {
    pub srcloc: u64,
    pub depth: ::std::os::raw::c_int,
    pub queryId: u16,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_zone_begin_callstack_data"]
        [::std::mem::size_of::<___tracy_gpu_zone_begin_callstack_data>() - 16usize];
    ["Alignment of ___tracy_gpu_zone_begin_callstack_data"]
        [::std::mem::align_of::<___tracy_gpu_zone_begin_callstack_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_zone_begin_callstack_data::srcloc"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_callstack_data, srcloc) - 0usize];
    ["Offset of field: ___tracy_gpu_zone_begin_callstack_data::depth"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_callstack_data, depth) - 8usize];
    ["Offset of field: ___tracy_gpu_zone_begin_callstack_data::queryId"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_callstack_data, queryId) - 12usize];
    ["Offset of field: ___tracy_gpu_zone_begin_callstack_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_zone_begin_callstack_data, context) - 14usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_zone_end_data {
    pub queryId: u16,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_zone_end_data"]
        [::std::mem::size_of::<___tracy_gpu_zone_end_data>() - 4usize];
    ["Alignment of ___tracy_gpu_zone_end_data"]
        [::std::mem::align_of::<___tracy_gpu_zone_end_data>() - 2usize];
    ["Offset of field: ___tracy_gpu_zone_end_data::queryId"]
        [::std::mem::offset_of!(___tracy_gpu_zone_end_data, queryId) - 0usize];
    ["Offset of field: ___tracy_gpu_zone_end_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_zone_end_data, context) - 2usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_new_context_data {
    pub gpuTime: i64,
    pub period: f32,
    pub context: u8,
    pub flags: u8,
    pub type_: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_new_context_data"]
        [::std::mem::size_of::<___tracy_gpu_new_context_data>() - 16usize];
    ["Alignment of ___tracy_gpu_new_context_data"]
        [::std::mem::align_of::<___tracy_gpu_new_context_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_new_context_data::gpuTime"]
        [::std::mem::offset_of!(___tracy_gpu_new_context_data, gpuTime) - 0usize];
    ["Offset of field: ___tracy_gpu_new_context_data::period"]
        [::std::mem::offset_of!(___tracy_gpu_new_context_data, period) - 8usize];
    ["Offset of field: ___tracy_gpu_new_context_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_new_context_data, context) - 12usize];
    ["Offset of field: ___tracy_gpu_new_context_data::flags"]
        [::std::mem::offset_of!(___tracy_gpu_new_context_data, flags) - 13usize];
    ["Offset of field: ___tracy_gpu_new_context_data::type_"]
        [::std::mem::offset_of!(___tracy_gpu_new_context_data, type_) - 14usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_context_name_data {
    pub context: u8,
    pub name: *const ::std::os::raw::c_char,
    pub len: u16,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_context_name_data"]
        [::std::mem::size_of::<___tracy_gpu_context_name_data>() - 24usize];
    ["Alignment of ___tracy_gpu_context_name_data"]
        [::std::mem::align_of::<___tracy_gpu_context_name_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_context_name_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_context_name_data, context) - 0usize];
    ["Offset of field: ___tracy_gpu_context_name_data::name"]
        [::std::mem::offset_of!(___tracy_gpu_context_name_data, name) - 8usize];
    ["Offset of field: ___tracy_gpu_context_name_data::len"]
        [::std::mem::offset_of!(___tracy_gpu_context_name_data, len) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_calibration_data {
    pub gpuTime: i64,
    pub cpuDelta: i64,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_calibration_data"]
        [::std::mem::size_of::<___tracy_gpu_calibration_data>() - 24usize];
    ["Alignment of ___tracy_gpu_calibration_data"]
        [::std::mem::align_of::<___tracy_gpu_calibration_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_calibration_data::gpuTime"]
        [::std::mem::offset_of!(___tracy_gpu_calibration_data, gpuTime) - 0usize];
    ["Offset of field: ___tracy_gpu_calibration_data::cpuDelta"]
        [::std::mem::offset_of!(___tracy_gpu_calibration_data, cpuDelta) - 8usize];
    ["Offset of field: ___tracy_gpu_calibration_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_calibration_data, context) - 16usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_gpu_time_sync_data {
    pub gpuTime: i64,
    pub context: u8,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_gpu_time_sync_data"]
        [::std::mem::size_of::<___tracy_gpu_time_sync_data>() - 16usize];
    ["Alignment of ___tracy_gpu_time_sync_data"]
        [::std::mem::align_of::<___tracy_gpu_time_sync_data>() - 8usize];
    ["Offset of field: ___tracy_gpu_time_sync_data::gpuTime"]
        [::std::mem::offset_of!(___tracy_gpu_time_sync_data, gpuTime) - 0usize];
    ["Offset of field: ___tracy_gpu_time_sync_data::context"]
        [::std::mem::offset_of!(___tracy_gpu_time_sync_data, context) - 8usize];
};
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __tracy_lockable_context_data {
    _unused: [u8; 0],
}
type TracyCZoneCtx = ___tracy_c_zone_context;
type TracyCLockCtx = *mut __tracy_lockable_context_data;
extern "C" {
    pub fn ___tracy_alloc_srcloc(
        line: u32,
        source: *const ::std::os::raw::c_char,
        sourceSz: usize,
        function: *const ::std::os::raw::c_char,
        functionSz: usize,
        color: u32,
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
        color: u32,
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
    pub fn ___tracy_emit_gpu_zone_begin(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_callstack(arg1: ___tracy_gpu_zone_begin_callstack_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc_callstack(
        arg1: ___tracy_gpu_zone_begin_callstack_data,
    );
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_end(data: ___tracy_gpu_zone_end_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time(arg1: ___tracy_gpu_time_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_new_context(arg1: ___tracy_gpu_new_context_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_context_name(arg1: ___tracy_gpu_context_name_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_calibration(arg1: ___tracy_gpu_calibration_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time_sync(arg1: ___tracy_gpu_time_sync_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_serial(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_callstack_serial(
        arg1: ___tracy_gpu_zone_begin_callstack_data,
    );
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc_serial(arg1: ___tracy_gpu_zone_begin_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_begin_alloc_callstack_serial(
        arg1: ___tracy_gpu_zone_begin_callstack_data,
    );
}
extern "C" {
    pub fn ___tracy_emit_gpu_zone_end_serial(data: ___tracy_gpu_zone_end_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time_serial(arg1: ___tracy_gpu_time_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_new_context_serial(arg1: ___tracy_gpu_new_context_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_context_name_serial(arg1: ___tracy_gpu_context_name_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_calibration_serial(arg1: ___tracy_gpu_calibration_data);
}
extern "C" {
    pub fn ___tracy_emit_gpu_time_sync_serial(arg1: ___tracy_gpu_time_sync_data);
}
extern "C" {
    pub fn ___tracy_connected() -> ::std::os::raw::c_int;
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
    pub fn ___tracy_emit_memory_alloc_named(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_alloc_callstack_named(
        ptr: *const ::std::os::raw::c_void,
        size: usize,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_named(
        ptr: *const ::std::os::raw::c_void,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
    );
}
extern "C" {
    pub fn ___tracy_emit_memory_free_callstack_named(
        ptr: *const ::std::os::raw::c_void,
        depth: ::std::os::raw::c_int,
        secure: ::std::os::raw::c_int,
        name: *const ::std::os::raw::c_char,
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
    pub fn ___tracy_emit_plot_float(name: *const ::std::os::raw::c_char, val: f32);
}
extern "C" {
    pub fn ___tracy_emit_plot_int(name: *const ::std::os::raw::c_char, val: i64);
}
extern "C" {
    pub fn ___tracy_emit_plot_config(
        name: *const ::std::os::raw::c_char,
        type_: ::std::os::raw::c_int,
        step: ::std::os::raw::c_int,
        fill: ::std::os::raw::c_int,
        color: u32,
    );
}
extern "C" {
    pub fn ___tracy_emit_message_appinfo(txt: *const ::std::os::raw::c_char, size: usize);
}
extern "C" {
    pub fn ___tracy_announce_lockable_ctx(
        srcloc: *const ___tracy_source_location_data,
    ) -> *mut __tracy_lockable_context_data;
}
extern "C" {
    pub fn ___tracy_terminate_lockable_ctx(lockdata: *mut __tracy_lockable_context_data);
}
extern "C" {
    pub fn ___tracy_before_lock_lockable_ctx(
        lockdata: *mut __tracy_lockable_context_data,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn ___tracy_after_lock_lockable_ctx(lockdata: *mut __tracy_lockable_context_data);
}
extern "C" {
    pub fn ___tracy_after_unlock_lockable_ctx(lockdata: *mut __tracy_lockable_context_data);
}
extern "C" {
    pub fn ___tracy_after_try_lock_lockable_ctx(
        lockdata: *mut __tracy_lockable_context_data,
        acquired: ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn ___tracy_mark_lockable_ctx(
        lockdata: *mut __tracy_lockable_context_data,
        srcloc: *const ___tracy_source_location_data,
    );
}
extern "C" {
    pub fn ___tracy_custom_name_lockable_ctx(
        lockdata: *mut __tracy_lockable_context_data,
        name: *const ::std::os::raw::c_char,
        nameSz: usize,
    );
}
