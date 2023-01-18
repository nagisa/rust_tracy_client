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
#![doc = include_str!("../FEATURES.mkd")]
#![allow(non_snake_case, non_camel_case_types, unused_variables, deref_nullptr)]

#[cfg(feature = "enable")]
mod generated;
#[cfg(feature = "enable")]
pub use generated::*;
