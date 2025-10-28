use crate::{span_location, sys, SpanLocation};

use std::fmt;
use std::mem::ManuallyDrop;
use std::ops::{Deref, DerefMut};
use std::sync::{LockResult, MutexGuard, PoisonError, TryLockError, TryLockResult};

/// A [std::sync::Mutex] with tracing
pub struct TracyMutex<T: ?Sized> {
    #[cfg(feature = "enable")]
    ctx: *mut sys::__tracy_lockable_context_data,

    inner: ManuallyDrop<std::sync::Mutex<T>>,
}

/// `T` must be `Send` for a [`Mutex`] to be `Send` because it is possible to acquire
/// the owned `T` from the `Mutex` via [`into_inner`].
///
/// [`into_inner`]: Mutex::into_inner
unsafe impl<T: ?Sized + Send> Send for TracyMutex<T> {}

/// `T` must be `Send` for [`Mutex`] to be `Sync`.
/// This ensures that the protected data can be accessed safely from multiple threads
/// without causing data races or other unsafe behavior.
///
/// [`Mutex<T>`] provides mutable access to `T` to one thread at a time. However, it's essential
/// for `T` to be `Send` because it's not safe for non-`Send` structures to be accessed in
/// this manner. For instance, consider [`Rc`], a non-atomic reference counted smart pointer,
/// which is not `Send`. With `Rc`, we can have multiple copies pointing to the same heap
/// allocation with a non-atomic reference count. If we were to use `Mutex<Rc<_>>`, it would
/// only protect one instance of `Rc` from shared access, leaving other copies vulnerable
/// to potential data races.
///
/// Also note that it is not necessary for `T` to be `Sync` as `&T` is only made available
/// to one thread at a time if `T` is not `Sync`.
///
/// [`Rc`]: crate::rc::Rc
unsafe impl<T: ?Sized + Send> Sync for TracyMutex<T> {}

/// An RAII implementation of a "scoped lock" of a mutex. When this structure is
/// dropped (falls out of scope), the lock will be unlocked.
///
/// The data protected by the mutex can be accessed through this guard via its
/// [`Deref`] and [`DerefMut`] implementations.
///
/// This structure is created by the [`lock`] and [`try_lock`] methods on
/// [`Mutex`].
///
/// [`lock`]: Mutex::lock
/// [`try_lock`]: Mutex::try_lock
#[must_use = "if unused the Mutex will immediately unlock"]
#[clippy::has_significant_drop]
pub struct TracyMutexGuard<'a, T: ?Sized + 'a> {
    lock: &'a TracyMutex<T>,
    inner: std::sync::MutexGuard<'a, T>,
}

/// A [`MutexGuard`] is not `Send` to maximize platform portability.
///
/// On platforms that use POSIX threads (commonly referred to as pthreads) there is a requirement to
/// release mutex locks on the same thread they were acquired.
/// For this reason, [`MutexGuard`] must not implement `Send` to prevent it being dropped from
/// another thread.
// impl<T: ?Sized> !Send for TracyMutexGuard<'_, T> {}

/// `T` must be `Sync` for a [`MutexGuard<T>`] to be `Sync`
/// because it is possible to get a `&T` from `&MutexGuard` (via `Deref`).
unsafe impl<T: ?Sized + Sync> Sync for TracyMutexGuard<'_, T> {}

impl<T> TracyMutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    ///
    /// The [`mutex!`](crate::mutex!) macro is a convenience wrapper over this method.
    ///
    #[inline]
    pub fn new(t: T, loc: &SpanLocation) -> TracyMutex<T> {
        let ctx = unsafe { sys::___tracy_announce_lockable_ctx(&loc.data) };
        TracyMutex {
            ctx,
            inner: ManuallyDrop::new(std::sync::Mutex::new(t)),
        }
    }
}

impl<T: ?Sized> TracyMutex<T> {
    /// See [std::sync::Mutex::lock]
    pub fn lock(&self) -> LockResult<TracyMutexGuard<'_, T>> {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_before_lock_lockable_ctx(self.ctx)
        };
        let res = self.inner.lock();
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_after_lock_lockable_ctx(self.ctx)
        };
        match res {
            Ok(inner) => Ok(TracyMutexGuard::new(self, inner)),
            Err(e) => Err(PoisonError::new(TracyMutexGuard::new(self, e.into_inner()))),
        }
    }

    /// See [std::sync::Mutex::try_lock]
    pub fn try_lock(&self) -> TryLockResult<TracyMutexGuard<'_, T>> {
        let res = self.inner.try_lock();
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_after_try_lock_lockable_ctx(self.ctx, res.is_ok() as i32)
        };
        match res {
            Ok(inner) => Ok(TracyMutexGuard::new(self, inner)),
            Err(TryLockError::Poisoned(e)) => Err(TryLockError::Poisoned(PoisonError::new(
                TracyMutexGuard::new(self, e.into_inner()),
            ))),
            Err(TryLockError::WouldBlock) => Err(TryLockError::WouldBlock),
        }
    }

    /// See [std::sync::Mutex::is_poisoned]
    #[inline]
    pub fn is_poisoned(&self) -> bool {
        self.inner.is_poisoned()
    }

    /// See [std::sync::Mutex::clear_poison]
    #[inline]
    pub fn clear_poison(&self) {
        self.inner.clear_poison();
    }

    /// See [std::sync::Mutex::into_inner]
    #[inline]
    pub fn into_inner(mut self) -> LockResult<T>
    where
        T: Sized,
    {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_terminate_lockable_ctx(self.ctx)
        };
        // Safety: TracyMutex Drop impl does not use this field.
        unsafe { ManuallyDrop::take(&mut self.inner).into_inner() }
    }

    /// See [std::sync::Mutex::get_mut]
    #[inline]
    pub fn get_mut(&mut self) -> LockResult<&mut T> {
        self.inner.get_mut()
    }
}

impl<T> From<T> for TracyMutex<T> {
    /// Creates a new mutex in an unlocked state ready for use.
    /// This is equivalent to [`TracyMutex::new`].
    fn from(t: T) -> Self {
        // TODO: Figure out where this was called from
        TracyMutex::new(t, span_location!())
    }
}

impl<T: ?Sized + Default> Default for TracyMutex<T> {
    /// Creates a `Mutex<T>`, with the `Default` value for T.
    fn default() -> TracyMutex<T> {
        // TODO: Figure out where this was called from
        TracyMutex::new(Default::default(), span_location!())
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for TracyMutex<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.inner.fmt(f)
    }
}

impl<T: ?Sized> Drop for TracyMutex<T> {
    #[inline]
    fn drop(&mut self) {
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_terminate_lockable_ctx(self.ctx)
        };
    }
}

impl<'mutex, T: ?Sized> TracyMutexGuard<'mutex, T> {
    fn new(
        lock: &'mutex TracyMutex<T>,
        inner: MutexGuard<'mutex, T>,
    ) -> TracyMutexGuard<'mutex, T> {
        TracyMutexGuard { lock, inner }
    }
}

impl<T: ?Sized> Deref for TracyMutexGuard<'_, T> {
    type Target = T;

    fn deref(&self) -> &T {
        self.inner.deref()
    }
}

impl<T: ?Sized> DerefMut for TracyMutexGuard<'_, T> {
    fn deref_mut(&mut self) -> &mut T {
        self.inner.deref_mut()
    }
}

impl<T: ?Sized> Drop for TracyMutexGuard<'_, T> {
    #[inline]
    fn drop(&mut self) {
        // TODO: Change the order so this is actually called after instead of before unlocking
        #[cfg(feature = "enable")]
        unsafe {
            sys::___tracy_after_unlock_lockable_ctx(self.lock.ctx)
        };
    }
}

impl<T: ?Sized + fmt::Debug> fmt::Debug for TracyMutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T: ?Sized + fmt::Display> fmt::Display for TracyMutexGuard<'_, T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        (**self).fmt(f)
    }
}

/// Create a new [TracyMutex] with function, file, and line determined automatically.
#[macro_export]
macro_rules! mutex {
    ($t:expr) => {
        TracyMutex::new($t, $crate::span_location!())
    };
    ($t:expr, $name: expr) => {{
        TracyMutex::new($t, $crate::span_location!($name))
    }};
}
