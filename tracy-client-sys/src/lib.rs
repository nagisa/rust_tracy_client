//! The Tracy Client and its low level API
//!
//! This crate embeds the C++ Tracy client library and exposes its API. For a higher-level API
//! consider the `tracy-client` crate.
//!
//! # Important note
//!
//! Depending on the configuration Tracy may broadcast discovery packets to the local network and
//! expose the data it collects in the background to that same network. Traces collected by Tracy
//! may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client-sys` crate conditionally, via
//! the `enable` feature flag provided by this crate.
//!
//! In order to start tracing it is important that you first call the [`___tracy_startup_profiler`]
//! function first to initialize the client. The [`___tracy_shutdown_profiler`] must not be called
//! until it is guaranteed that there will be no more calls to any other Tracy APIs. This can be
//! especially difficult to ensure if you have detached threads.
//!
//! # Features
//!
//! The following crate features are provided to customize the functionality of the Tracy client:
//!
//! * `enable` – enables the Tracy client. Corresponds to the `TRACY_ENABLE` define.
//! * `system-tracing` – enable capture of system level details. Corresponds to the
//!   `TRACY_NO_SYSTEM_TRACING` define.
//! * `context-switch-tracing` – enable capture of the context switch data. Corresponds to the
//!   `TRACY_NO_CONTEXT_SWITCH` define.
//! * `sampling` – enable periodic sampling of the call stack. Corresponds to the
//!   `TRACY_NO_SAMPLING` define.
//! * `code-transfer` – enable transfer of the machine code to the profiler. Corresponds to the
//!   `TRACY_NO_CODE_TRANSFER` define.
//! * `broadcast` – announce presence of the client to the profilers on the local network.
//!   Corresponds to the `TRACY_NO_BROADCAST` define.
//! * `only-localhost` – listen for profilers on the localhost interface only. Corresponds to the
//!   `TRACY_ONLY_LOCALHOST` define.
//! * `only-ipv4` – listen for profilers on IPv4 interfaces only. Corresponds to the
//!   `TRACY_ONLY_IPV4` define.
//! * `timer-fallback` – allow running on devices without a high resolution timer support.
//!   Corresponds to the `TRACY_TIMER_FALLBACK` define.
//! * `ondemand` – start collecting traces only when a server connects to the client. Corresponds
//!   to the `TRACY_ON_DEMAND` define.
//! * `fibers` – enable support for instrumenting fibers, coroutines and similar such asynchrony
//!   primitives. Corresponds to the `TRACY_FIBERS` define.
//!
//! Refer to this package's `Cargo.toml` for the list of the features enabled by default. Refer to
//! the `Tracy` manual for more information on the implications of each feature.
#![allow(non_snake_case, non_camel_case_types, unused_variables, deref_nullptr)]

#[cfg(feature = "enable")]
mod generated;
#[cfg(feature = "enable")]
pub use generated::*;
