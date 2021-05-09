//! Low level API to the Tracy Client
//!
//! This crate embeds the C++ tracy client library and exposes its API. For a higher-level API
//! consider `tracy-client`.
//!
//! # Important note
//!
//! Simply depending on this crate is sufficient for tracy to be enabled at program startup, even
//! if none of the APIs provided by this crate are invoked. Tracy will broadcast discovery packets
//! to the local network and expose the data it collects in the background to that same network.
//! Traces collected by Tracy may include source and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client-sys` crate conditionally, via
//! the `enable` feature flag provided by this crate.
#![allow(non_snake_case, non_camel_case_types, unused_variables)]

#[cfg(feature="enable")]
mod generated;
#[cfg(feature="enable")]
pub use generated::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn emit_zone() {
        unsafe {
            ___tracy_init_thread();
            let srcloc = ___tracy_source_location_data {
                name: b"name\0".as_ptr() as _,
                function: b"function\0".as_ptr() as _,
                file: b"file\0".as_ptr() as _,
                line: 42,
                color: 0,
            };
            let zone_ctx = ___tracy_emit_zone_begin(&srcloc, 1);
            ___tracy_emit_zone_end(zone_ctx);
        }
    }

    #[test]
    fn emit_message_no_null() {
        unsafe {
            ___tracy_init_thread();
            ___tracy_emit_message(b"hello world".as_ptr() as _, 11, 1);
        }
    }
}
