#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct ___tracy_c_zone_context {
    pub id: u32,
    pub active: i32,
    pub connectionId: u64,
}
#[test]
fn bindgen_test_layout____tracy_c_zone_context() {
    const UNINIT: ::std::mem::MaybeUninit<___tracy_c_zone_context> =
        ::std::mem::MaybeUninit::uninit();
    let ptr = UNINIT.as_ptr();
    assert_eq!(
        ::std::mem::size_of::<___tracy_c_zone_context>(),
        16usize,
        "Size of ___tracy_c_zone_context"
    );
    assert_eq!(
        ::std::mem::align_of::<___tracy_c_zone_context>(),
        8usize,
        "Alignment of ___tracy_c_zone_context"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).id) as usize - ptr as usize },
        0usize,
        "Offset of field: ___tracy_c_zone_context::id"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).active) as usize - ptr as usize },
        4usize,
        "Offset of field: ___tracy_c_zone_context::active"
    );
    assert_eq!(
        unsafe { ::std::ptr::addr_of!((*ptr).connectionId) as usize - ptr as usize },
        8usize,
        "Offset of field: ___tracy_c_zone_context::connectionId"
    );
}
