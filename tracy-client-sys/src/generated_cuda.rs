/*
The real generated file was made like so:

bindgen -o "tracy-client-sys/src/generated_cuda.rs" --rust-target "1.70.0" --allowlist-function='.*tracy::CUDACtx.*' --blocklist-type='TracyCLockCtx' "tracy-client-sys/tracy/tracy/TracyCUDA.hpp" --no-size_t-is-usize --generate-inline-functions -- -xc++ -DTRACY_ENABLE -I/opt/cuda/targets/x86_64-linux/include -I/opt/cuda/extras/CUPTI/include -Itracy-client-sys/tracy --target=x86_64-unknown-linux-gnu

it contains _way_ too much stuff, including stuff that doesn't compile due to strange type definitions
including hash maps and generics.

For just testing the simplest thing to do was to just perform surgery to keep only the necessary parts.
Which essentially is
    - pub fn tracy_CUDACtx_Create() -> *mut tracy_CUDACtx;
    - pub fn tracy_CUDACtx_StartProfiling(ctx: *mut tracy_CUDACtx);

The other code relating to threads and mutexes etc. is just there to make
the program compile since tracy_CUDACtx needs them.
*/

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct __pthread_mutex_s {
    pub __lock: ::std::os::raw::c_int,
    pub __count: ::std::os::raw::c_uint,
    pub __owner: ::std::os::raw::c_int,
    pub __nusers: ::std::os::raw::c_uint,
    pub __kind: ::std::os::raw::c_int,
    pub __spins: ::std::os::raw::c_short,
    pub __elision: ::std::os::raw::c_short,
    pub __list: __pthread_list_t,
}
#[repr(C)]
#[derive(Copy, Clone)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [::std::os::raw::c_char; 40usize],
    pub __align: ::std::os::raw::c_long,
}
pub type __gthread_mutex_t = pthread_mutex_t;
pub type std___mutex_base___native_type = __gthread_mutex_t;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct std___mutex_base {
    pub _M_mutex: std___mutex_base___native_type,
}
extern "C" {
    #[link_name = "\u{1}_ZNSt12__mutex_baseC1Ev"]
    pub fn std___mutex_base___mutex_base(this: *mut std___mutex_base);
}
impl std___mutex_base {
    #[inline]
    pub unsafe fn new() -> Self {
        let mut __bindgen_tmp = ::std::mem::MaybeUninit::uninit();
        std___mutex_base___mutex_base(__bindgen_tmp.as_mut_ptr());
        __bindgen_tmp.assume_init()
    }
}
#[repr(C)]
pub struct std_mutex {
    pub _base: std___mutex_base,
}
#[repr(C)]
#[derive(Debug)]
pub struct std_atomic<_Tp> {
    pub _phantom_0: ::std::marker::PhantomData<::std::cell::UnsafeCell<_Tp>>,
    pub _M_i: _Tp,
}
#[repr(C)]
#[repr(align(64))]
#[derive(Debug)]
pub struct tracy_CUDACtx {
    pub m_tracyGpuContext: u8,
    pub __bindgen_padding_0: [u16; 31usize],
    pub m_queryIdGen: std_atomic<u16>,
}
#[repr(C)]
pub struct tracy_CUDACtx_Singleton {
    pub ctx: *mut tracy_CUDACtx,
    pub m: std_mutex,
    pub ref_count: ::std::os::raw::c_int,
    pub ctx_id: u8,
}

extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx9Singleton3GetEv"]
    pub fn tracy_CUDACtx_Singleton_Get() -> *mut tracy_CUDACtx_Singleton;
}
impl tracy_CUDACtx_Singleton {
    #[inline]
    pub unsafe fn Get() -> *mut tracy_CUDACtx_Singleton {
        tracy_CUDACtx_Singleton_Get()
    }
}

extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx6CreateEv"]
    pub fn tracy_CUDACtx_Create() -> *mut tracy_CUDACtx;
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx7DestroyEPS0_"]
    pub fn tracy_CUDACtx_Destroy(ctx: *mut tracy_CUDACtx);
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx7CollectEv"]
    pub fn tracy_CUDACtx_Collect(this: *mut tracy_CUDACtx);
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx10printStatsEv"]
    pub fn tracy_CUDACtx_printStats(this: *mut tracy_CUDACtx);
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx14StartProfilingEv"]
    pub fn tracy_CUDACtx_StartProfiling(this: *mut tracy_CUDACtx);
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx13StopProfilingEv"]
    pub fn tracy_CUDACtx_StopProfiling(this: *mut tracy_CUDACtx);
}
extern "C" {
    #[link_name = "\u{1}_ZN5tracy7CUDACtx4NameEPKct"]
    pub fn tracy_CUDACtx_Name(
        this: *mut tracy_CUDACtx,
        name: *const ::std::os::raw::c_char,
        len: u16,
    );
}
