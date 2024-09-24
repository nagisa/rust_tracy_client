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

use std::io::Write;
use std::sync::atomic::{AtomicPtr, Ordering};
use windows::core::PCSTR;
use windows::Win32::Foundation::{FALSE, HANDLE};
use windows::Win32::System::Threading::{
    CreateMutexA, GetCurrentProcessId, ReleaseMutex, WaitForSingleObject, INFINITE,
};

/// Handle to the shared named Windows mutex that synchronizes access to the `dbghelp.dll` symbol helper,
/// with the standard library and `backtrace-rs`.
/// Gets initialized by [`RustBacktraceMutexInit`],
/// and because there is no cleanup function, the handle is leaked.
static RUST_BACKTRACE_MUTEX: AtomicPtr<core::ffi::c_void> = AtomicPtr::new(std::ptr::null_mut());

#[no_mangle]
extern "C" fn RustBacktraceMutexInit() {
    unsafe {
        // The name is the same one that the standard library and `backtrace-rs` use
        let mut name = [0; 33];
        let id = GetCurrentProcessId();
        write!(&mut name[..], "Local\\RustBacktraceMutex{id:08X}\0").unwrap();
        let name = PCSTR::from_raw(name.as_ptr());

        // Creates a named mutex that is shared with the standard library and `backtrace-rs`
        // to synchronize access to `dbghelp.dll` functions, which are single threaded.
        let mutex = CreateMutexA(None, FALSE, name).unwrap();
        assert!(!mutex.is_invalid());

        // The old value is ignored because this function is only called once,
        // and normally the handle to the mutex is leaked anyway.
        RUST_BACKTRACE_MUTEX.store(mutex.0, Ordering::Release);
    }
}

#[no_mangle]
extern "C" fn RustBacktraceMutexLock() {
    unsafe {
        let mutex = HANDLE(RUST_BACKTRACE_MUTEX.load(Ordering::Acquire));
        assert!(!mutex.is_invalid());
        WaitForSingleObject(mutex, INFINITE);
    }
}

#[no_mangle]
extern "C" fn RustBacktraceMutexUnlock() {
    unsafe {
        let mutex = HANDLE(RUST_BACKTRACE_MUTEX.load(Ordering::Acquire));
        assert!(!mutex.is_invalid());
        ReleaseMutex(mutex).unwrap();
    }
}
