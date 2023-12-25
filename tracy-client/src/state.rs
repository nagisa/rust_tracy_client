use crate::Client;
use std::sync::atomic::Ordering;

/// Enabling `Tracy` when it is already enabled, or Disabling when it is already disabled will
/// cause applications to crash. I personally think it would be better if this was a sort-of
/// reference counted kind-of thing so you could enable as many times as you wish and disable
/// just as many times without any reprecursions. At the very least this could significantly
/// help tests.
///
/// We can also try to implement something like this ourselves. To do this we'd want to track 4
/// states that construct a following finite state machine:
///
/// ```text
///     0 = disabled  -> 1 = enabling
///         ^                v
///     3 = disabling <- 2 = enabled
/// ```
///
/// And also include a reference count somewhere in there. Something we can throw in a static
/// would be ideal.
///
/// Alas, Tracy's extensive use of thread-local storage presents us with another problem – we must
/// start up and shut down the client within the same thread. A most straightforward soution for
/// that would be to run a separate thread that would be dedicated entirely to just starting up and
/// shutting down the profiler.
///
/// All that seems like a major pain to implement, and so we’ll punt on disabling entirely until
/// somebody comes with a good use-case warranting that sort of complexity.
#[cfg(feature = "enable")]
#[cfg(not(loom))]
static CLIENT_STATE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(loom)]
loom::lazy_static! {
    static ref CLIENT_STATE: loom::sync::atomic::AtomicUsize =
        loom::sync::atomic::AtomicUsize::new(0);
}

#[cfg(feature = "enable")]
const STATE_STEP: usize = 1; // Move forward by 1 step in the FSM
#[cfg(feature = "enable")]
const STATE_DISABLED: usize = 0;
#[cfg(feature = "enable")]
const STATE_ENABLING: usize = STATE_DISABLED + STATE_STEP;
#[cfg(feature = "enable")]
const STATE_ENABLED: usize = STATE_ENABLING + STATE_STEP;

#[cfg(feature = "enable")]
#[inline(always)]
fn spin_loop() {
    #[cfg(loom)]
    loom::thread::yield_now();
    #[cfg(not(loom))]
    std::hint::spin_loop();
}

/// Client initialization and lifetime management.
impl Client {
    /// Start the client.
    ///
    /// The client must be started with this function before any instrumentation is invoked
    /// anywhere in the process. This function can be called multiple times to obtain multiple
    /// `Client` values.
    ///
    /// The underying client implementation will be started up only if it wasn't already running
    /// yet.
    ///
    /// Note that there currently isn't a mechanism to stop the client once it has been started.
    ///
    /// # Example
    ///
    /// ```rust
    /// // fn main() {
    ///     let _client = tracy_client::Client::start();
    ///     // ...
    /// // }
    /// ```
    pub fn start() -> Self {
        #[cfg(feature = "enable")]
        {
            let mut old_state = CLIENT_STATE.load(Ordering::Relaxed);
            loop {
                match old_state {
                    STATE_ENABLED => return Self(()),
                    STATE_ENABLING => {
                        while !Self::is_running() {
                            spin_loop();
                        }
                        return Self(());
                    }
                    STATE_DISABLED => {
                        let result = CLIENT_STATE.compare_exchange_weak(
                            old_state,
                            STATE_ENABLING,
                            Ordering::Relaxed,
                            Ordering::Relaxed,
                        );
                        if let Err(next_old_state) = result {
                            old_state = next_old_state;
                            continue;
                        } else {
                            unsafe {
                                // SAFE: This function must not be called if the profiler has
                                // already been enabled. While in practice calling this function
                                // multiple times will only serve to trigger an assertion, we
                                // cannot exactly rely on this, since it is an undocumented
                                // behaviour and the upstream might very well just decide to invoke
                                // UB instead. In the case there are multiple copies of
                                // `tracy-client` this invariant is not actually maintained, but
                                // otherwise this is sound due to the `ENABLE_STATE` that we
                                // manage.
                                //
                                // TODO: we _could_ define `ENABLE_STATE` in the `-sys` crate...
                                sys::___tracy_startup_profiler();
                                CLIENT_STATE.store(STATE_ENABLED, Ordering::Release);
                                return Self(());
                            }
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
        #[cfg(not(feature = "enable"))]
        Client(())
    }

    /// Obtain a client handle, but only if the client is already running.
    #[inline(always)]
    #[must_use]
    pub fn running() -> Option<Self> {
        if Self::is_running() {
            Some(Self(()))
        } else {
            None
        }
    }

    /// Is the client already running?
    #[inline(always)]
    pub fn is_running() -> bool {
        #[cfg(feature = "enable")]
        return CLIENT_STATE.load(Ordering::Relaxed) == STATE_ENABLED;
        #[cfg(not(feature = "enable"))]
        return true;
    }
}

impl Clone for Client {
    /// A cheaper alternative to [`Client::start`] or [`Client::running`]  when there is already a
    /// handle handy.
    #[inline(always)]
    fn clone(&self) -> Self {
        // We already know that the state is `ENABLED`, no need to check.
        Self(())
    }
}

#[cfg(all(test, feature = "enable"))]
mod test {
    use super::*;

    #[test]
    fn state_transitions() {
        assert_eq!(0, STATE_DISABLED);
        assert_eq!(STATE_DISABLED.wrapping_add(STATE_STEP), STATE_ENABLING);
        assert_eq!(STATE_ENABLING.wrapping_add(STATE_STEP), STATE_ENABLED);
    }
}
