use crate::Client;

/// A name for a Tracy fiber.
///
/// Create with the [`fiber_name!`](crate::fiber_name) macro.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct FiberName(pub(crate) &'static str);

impl FiberName {
    /// Construct a `FiberName` dynamically, leaking the provided String.
    ///
    /// You should call this function once for a given name, and store the returned `FiberName` for
    /// continued use, to avoid rapid memory use growth. Whenever possible, prefer the
    /// [`fiber_name!`](crate::fiber_name) macro, which takes a literal name and doesn't leak
    /// memory.
    #[must_use]
    pub fn new_leak(name: String) -> Self {
        #[cfg(feature = "enable")]
        {
            let mut name = name;
            name.push('\0');
            let name = Box::leak(name.into_boxed_str());
            Self(name)
        }
        #[cfg(not(feature = "enable"))]
        {
            drop(name);
            Self("\0")
        }
    }
}

/// Instrumentation for fibers, coroutines, and similar cooperative multitasking primitives.
///
/// Tracy fibers allow attributing zones and messages to a logical fiber rather than to the
/// underlying OS thread. This is useful for profiling async runtimes where multiple tasks
/// share a single thread.
impl Client {
    /// Enter a fiber context.
    ///
    /// All subsequent zones, messages, and other events on the current thread will be attributed
    /// to the named fiber until [`Client::fiber_leave`] is called or another fiber is entered.
    ///
    /// It is valid to call `fiber_enter` multiple times without an intermediate `fiber_leave`
    /// call; this represents a direct context switch between fibers.
    pub fn fiber_enter(&self, name: FiberName) {
        #[cfg(all(feature = "enable", feature = "fibers"))]
        unsafe {
            let () = sys::___tracy_fiber_enter(name.0.as_ptr().cast());
        }
        #[cfg(not(all(feature = "enable", feature = "fibers")))]
        let _ = name;
    }

    /// Leave the current fiber context.
    ///
    /// Subsequent events will be attributed to the current OS thread.
    pub fn fiber_leave(&self) {
        #[cfg(all(feature = "enable", feature = "fibers"))]
        unsafe {
            let () = sys::___tracy_fiber_leave();
        }
    }
}

/// Construct a [`FiberName`].
///
/// The resulting value may be used as an argument for the [`Client::fiber_enter`] method.
/// The macro can be used in a `const` context.
///
/// # Example
///
/// ```rust
/// let name: tracy_client::FiberName = tracy_client::fiber_name!("my-fiber");
/// ```
#[macro_export]
macro_rules! fiber_name {
    ($name: literal) => {{
        unsafe { $crate::internal::create_fiber_name(concat!($name, "\0")) }
    }};
}

/// Convenience shortcut for [`Client::fiber_enter`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
pub fn fiber_enter(name: FiberName) {
    Client::running()
        .expect("fiber_enter without a running Client")
        .fiber_enter(name);
}

/// Convenience shortcut for [`Client::fiber_leave`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
pub fn fiber_leave() {
    Client::running()
        .expect("fiber_leave without a running Client")
        .fiber_leave();
}
