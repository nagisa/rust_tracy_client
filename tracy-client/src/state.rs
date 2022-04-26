use crate::Client;
use std::sync::atomic::Ordering;

/// Enabling `Tracy` when it is already enabled, or Disabling when it is already disabled will
/// cause applications to crash. I personally think it would be better if this was a sort-of
/// reference counted kind-of thing so you could enable as many times as you wish and disable
/// just as many times without any reprecursions. At the very least this could significantly
/// help tests.
///
/// In order to do this we want to track 4 states that construct a following finite state
/// machine
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
/// Sadly, I am not aware of any libraries which would make this easier, so rolling out our own
/// it is then!
///
/// Oh and it'll definitely involve spinning, getting some vertigo medication is advised.
///
/// <...ommitted...>
///
/// We use a single Atomic to store this information. The 2 top-most bits will represent
/// the state bits and the rest will act as a counter.
#[cfg(feature = "enable")]
#[cfg(not(loom))]
static ENABLE_STATE: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
#[cfg(loom)]
loom::lazy_static! {
    static ref ENABLE_STATE: loom::sync::atomic::AtomicUsize =
        loom::sync::atomic::AtomicUsize::new(0);
}

#[cfg(feature = "enable")]
const REFCOUNT_MASK: usize = usize::max_value() >> 2;
#[cfg(feature = "enable")]
const STATE_STEP: usize = REFCOUNT_MASK + 1; // Move forward by 1 step in the FSM
#[cfg(feature = "enable")]
const STATE_DISABLED: usize = 0;
#[cfg(feature = "enable")]
const STATE_ENABLING: usize = STATE_DISABLED + STATE_STEP;
#[cfg(feature = "enable")]
const STATE_ENABLED: usize = STATE_ENABLING + STATE_STEP;
#[cfg(feature = "enable")]
const STATE_DISABLING: usize = STATE_ENABLED + STATE_STEP;
#[cfg(feature = "enable")]
const STATE_MASK: usize = STATE_DISABLING;

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
    /// The client must be started with this function before any instrumentation may be invoked.
    /// This function can be called multiple times to obtain multiple `Client` values. Doing so
    /// will increase a counter indicating number of active client values.
    ///
    /// The underying client implementation will be started up only if it wasn't already running
    /// yet.
    ///
    /// Note that for heavily contended invocations of this method and [`Client::close`], the
    /// implementation is biased towards keeping the client running.
    ///
    /// # Example
    ///
    /// ```rust
    ///     let client = tracy_client::Client::enable();
    ///     // ...
    ///     drop(client);
    /// ```
    pub fn start() -> Self {
        #[cfg(feature = "enable")]
        {
            let old_state = loop {
                let result =
                    ENABLE_STATE.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |state| {
                        // Here we want to increment the reference count, and also apply the
                        // tansition from state 0 (disabled) to state 1 (enabling), if possible.
                        match state & STATE_MASK {
                            STATE_DISABLED => Some(STATE_ENABLING + 1),
                            STATE_ENABLED | STATE_ENABLING => Some(state + 1),
                            // Wait for the ongoing disable to complete.
                            STATE_DISABLING => None,
                            _ => unreachable!(),
                        }
                    });
                if let Ok(result) = result {
                    break result;
                }
                spin_loop();
            };
            match old_state & STATE_MASK {
                STATE_DISABLED => {
                    unsafe {
                        // SAFE: This function must not be called if the profiler has already
                        // been enabled. While in practice calling this function multiple times
                        // will only serve to trigger an assertion, we cannot exactly rely on this,
                        // since it is an undocumented behaviour and the upstream might very well
                        // just decide to invoke UB instead. In the case there are multiple copies
                        // of `tracy-client` this invariant is not actually maintained , but
                        // otherwise this is sound due to the `ENABLE_STATE` that we manage.
                        //
                        // TODO: we _could_ define `ENABLE_STATE` in the `-sys` crate...
                        sys::___tracy_startup_profiler();
                        // `STATE_DISABLED` became `STATE_ENABLING` in the `fetch_update` loop
                        // above. Now that the profile has been started, step the state forward
                        // once more so that it becomes a `STATE_ENABLED`.
                        ENABLE_STATE.fetch_add(STATE_STEP, Ordering::Release);
                        Client(())
                    }
                }
                STATE_ENABLING => {
                    // Something else is already enabling the profiler. Wait for that to finish.
                    while ENABLE_STATE.load(Ordering::Relaxed) & STATE_MASK != STATE_ENABLED {
                        spin_loop();
                    }
                    Client(())
                }
                STATE_ENABLED => Client(()),
                // The `fetch_update` loop cannot terminate in a DISABLING state.
                _ => unreachable!(),
            }
        }
        #[cfg(not(feature = "enable"))]
        Client(())
    }

    /// Obtain a client handle, but only if the client is already running.
    pub fn running() -> Option<Self> {
        ENABLE_STATE
            .fetch_update(Ordering::Relaxed, Ordering::Relaxed, |state| {
                match state & STATE_MASK {
                    STATE_ENABLED => Some(state + 1),
                    STATE_DISABLING | STATE_ENABLING | STATE_DISABLED => None,
                    _ => unreachable!(),
                }
            })
            .ok()
            .map(|_| Client(()))
    }

    /// Is the client already running?
    pub fn is_running() -> bool {
        ENABLE_STATE.load(Ordering::Relaxed) & STATE_MASK == STATE_ENABLED
    }
}

impl Clone for Client {
    /// A cheaper alternative to [`Client::start`] or [`Client::running`]  when there is already a
    /// handle handy.
    fn clone(&self) -> Self {
        // We already know that the state is `ENABLED`, so simply increment the counter.
        #[cfg(feature = "enable")]
        ENABLE_STATE.fetch_add(1, Ordering::Relaxed);
        Client(())
    }
}

/// Relinquish this Client handle.
///
/// If this is the last live handle, the client will be disabled. Once the client has been
/// disabled, no other calls to the instrumentation APIs may be made. Note that unloading the
/// client will also discard any data collected up to that point.
///
/// When using threads, especially detached ones, consider never calling `close` on at least
/// one of the handles, or at least use a Client handle for each thread that may be detached.
impl Drop for Client {
    fn drop(&mut self) {
        #[cfg(feature = "enable")]
        {
            let mut disable = false;
            let _ = ENABLE_STATE.fetch_update(Ordering::Relaxed, Ordering::Relaxed, |state| {
                // This is only reachable if the state is currently `STATE_ENABLED` and there's
                // at least 1 remaining reference count.
                Some(if state & REFCOUNT_MASK == 1 {
                    disable = true;
                    STATE_DISABLING
                } else {
                    disable = false;
                    state - 1
                })
            });
            if disable {
                unsafe {
                    // SAFE: we made sure to ensure there won't be any more instrumentations
                    // from at least this library.
                    sys::___tracy_shutdown_profiler();
                }
                ENABLE_STATE.store(0, Ordering::Release);
            }
        }
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
        assert_eq!(STATE_ENABLED.wrapping_add(STATE_STEP), STATE_DISABLING);
        assert_eq!(STATE_DISABLING.wrapping_add(STATE_STEP), STATE_DISABLED);
        assert_eq!(
            (STATE_DISABLED | STATE_ENABLING | STATE_ENABLED | STATE_DISABLING) & REFCOUNT_MASK,
            0
        );
    }
}
