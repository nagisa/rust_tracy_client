//! On Windows, both Tracy and Rust use the `dbghelp.dll` symbol helper to resolve symbols for stack traces.
//! `dbghelp.dll` is single threaded and requires synchronization to call any of its functions.
//!
//! The Rust standard library includes the `backtrace-rs` crate for capturing and resolving backtraces.
//! When both the standard library and the `backtrace-rs` crate are used in the same program
//! they need to synchronize their access to `dbghelp.dll`.
//! They use a shared named Windows mutex for that, which we will use as well.
//!
//! Users of Tracy (like this crate) can define the `TRACY_DBGHELP_LOCK` variable for synchronizing access to `dbghelp.dll`.
//! We set `TRACY_DBGHELP_LOCK=RustBacktraceMutex` in the build script.
//! Tracy will call [`RustBacktraceMutexInit`], [`RustBacktraceMutexLock`], and [`RustBacktraceMutexUnlock`].
//! In those functions a handle to the shared named mutex is created, the mutex is locked, and unlocked respectively.
//!
//! There is also an issue with initialization between Tracy and `backtrace-rs`.
//! In particular, the `SymInitialize` function should only be called once per process
//! and will return an error on subsequent calls.
//! Both Tracy and `backtrace-rs` ignore errors of the `SymInitialize` function,
//! so calling it multiple times is not an issue.

use std::sync::atomic::{AtomicPtr, Ordering};

// Use the `windows_targets` crate and define all the things we need ourselves to avoid a dependency on `windows`
#[allow(clippy::upper_case_acronyms)]
type BOOL = i32;
#[allow(clippy::upper_case_acronyms)]
type HANDLE = *mut core::ffi::c_void;
#[allow(clippy::upper_case_acronyms)]
type PCSTR = *const u8;
type WIN32_ERROR = u32;
#[repr(C)]
struct SECURITY_ATTRIBUTES {
    nLength: u32,
    lpSecurityDescriptor: *mut core::ffi::c_void,
    bInheritHandle: BOOL,
}

const FALSE: BOOL = 0i32;
const INFINITE: u32 = u32::MAX;
const WAIT_FAILED: u32 = 0xFFFFFFFF;

windows_targets::link!("kernel32.dll" "system" fn GetCurrentProcessId() -> u32);
windows_targets::link!("kernel32.dll" "system" fn CreateMutexA(lpmutexattributes: *const SECURITY_ATTRIBUTES, binitialowner: BOOL, lpname: PCSTR) -> HANDLE);
windows_targets::link!("kernel32.dll" "system" fn GetLastError() -> WIN32_ERROR);
windows_targets::link!("kernel32.dll" "system" fn WaitForSingleObject(hhandle: HANDLE, dwmilliseconds: u32) -> u32);
windows_targets::link!("kernel32.dll" "system" fn ReleaseMutex(hmutex: HANDLE) -> BOOL);

/// Handle to the shared named Windows mutex that synchronizes access to the `dbghelp.dll` symbol helper,
/// with the standard library and `backtrace-rs`.
/// Gets initialized by [`RustBacktraceMutexInit`],
/// and because there is no cleanup function, the handle is leaked.
static RUST_BACKTRACE_MUTEX: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
extern "C" fn RustBacktraceMutexInit() {
    unsafe {
        // The name is the same one that the standard library and `backtrace-rs` use
        let name = format!("Local\\RustBacktraceMutex{:08X}\0", GetCurrentProcessId());
        // Creates a named mutex that is shared with the standard library and `backtrace-rs`
        // to synchronize access to `dbghelp.dll` functions, which are single threaded.
        let mutex = CreateMutexA(std::ptr::null(), FALSE, name.as_ptr());
        assert!(!mutex.is_null());

        // The old value is ignored because this function is only called once,
        // and normally the handle to the mutex is leaked anyway.
        RUST_BACKTRACE_MUTEX.store(mutex, Ordering::Release);
    }
}

#[no_mangle]
extern "C" fn RustBacktraceMutexLock() {
    unsafe {
        let mutex = RUST_BACKTRACE_MUTEX.load(Ordering::Acquire);
        if !mutex.is_null() {
            assert_ne!(
                WaitForSingleObject(mutex, INFINITE),
                WAIT_FAILED,
                "{}",
                GetLastError()
            );
        }
    }
}

#[no_mangle]
extern "C" fn RustBacktraceMutexUnlock() {
    unsafe {
        let mutex = RUST_BACKTRACE_MUTEX.load(Ordering::Acquire);
        if !mutex.is_null() {
            assert_ne!(ReleaseMutex(mutex), 0, "{}", GetLastError());
        }
    }
}
