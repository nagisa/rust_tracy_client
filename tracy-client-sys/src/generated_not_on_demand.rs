#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: i32,
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
