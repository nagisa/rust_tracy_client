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
//! But `backtrace-rs` adds `SYMOPT_DEFERRED_LOADS` to the symbol options before initialization,
//! and adds the directory of all loaded modules (executable and DLLs) to the symbol search path.
//! That causes the symbols for Rust modules to be found even when the working directory isn't the Cargo target directory.
//! Tracy doesn't add the `SYMOPT_DEFERRED_LOADS` option and manually loads all modules.
//! Note that changing the symbol search path doesn't affect modules that were already loaded.
//!
//! Therefore, we want `backtrace-rs` to initialize and modify the symbol search path before Tracy.
//! To do that, a standard library backtrace is captured and resolved in [`RustBacktraceMutexInit`].

use std::io::{sink, Write};
use std::sync::atomic::{AtomicPtr, Ordering};
use windows::core::PCSTR;
use windows::Win32::Foundation::{GetLastError, ERROR_ALREADY_EXISTS, FALSE, HANDLE};
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
        // Initialize the `dbghelp.dll` symbol helper by capturing and resolving a backtrace using the standard library.
        // Since symbol resolution is lazy, the backtrace is written to `sink`, which forces symbol resolution.
        // Refer to the module documentation on why the standard library should do the initialization instead of Tracy.
        write!(sink(), "{:?}", std::backtrace::Backtrace::force_capture()).unwrap();

        // The name is the same one that the standard library and `backtrace-rs` use
        let mut name = [0; 33];
        let id = GetCurrentProcessId();
        write!(&mut name[..], "Local\\RustBacktraceMutex{id:08X}\0").unwrap();
        let name = PCSTR::from_raw(name.as_ptr());

        // Creates a named mutex that is shared with the standard library and `backtrace-rs`
        // to synchronize access to `dbghelp.dll` functions, which are single threaded.
        let mutex = CreateMutexA(None, FALSE, name).unwrap();
        assert!(!mutex.is_invalid());

        // Initialization of the `dbghelp.dll` symbol helper should have already happened
        // through the standard library backtrace above.
        // Therefore, the shared named mutex should already have existed.
        assert_eq!(GetLastError(), ERROR_ALREADY_EXISTS);

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