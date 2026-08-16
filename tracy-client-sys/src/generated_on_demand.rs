#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: i32,
    pub connectionId: u64,
}
#[allow(clippy::unnecessary_operation, clippy::identity_op)]
const _: () = {
    ["Size of ___tracy_c_zone_context"][::std::mem::size_of::<___tracy_c_zone_context>() - 16usize];
    ["Alignment of ___tracy_c_zone_context"]
        [::std::mem::align_of::<___tracy_c_zone_context>() - 8usize];
    ["Offset of field: ___tracy_c_zone_context::id"]
        [::std::mem::offset_of!(___tracy_c_zone_context, id) - 0usize];
    ["Offset of field: ___tracy_c_zone_context::active"]
        [::std::mem::offset_of!(___tracy_c_zone_context, active) - 4usize];
    ["Offset of field: ___tracy_c_zone_context::connectionId"]
        [::std::mem::offset_of!(___tracy_c_zone_context, connectionId) - 8usize];
};
