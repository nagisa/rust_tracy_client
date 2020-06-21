//! This crate is a set of safe bindings to the client library of the [Tracy profiler].
//!
//! If you have already instrumented your application with `tracing`, consider `tracing-tracy`.
//!
//! # Important note
//!
//! Simply depending on this crate is sufficient
//! for tracy to be enabled at program startup, even if none of the APIs provided by this crate are
//! invoked. Tracy will broadcast discovery packets to the local network and expose the data it
//! collects in the background to that same network. Traces collected by Tracy may include source
//! and assembly code as well.
//!
//! As thus, you may want make sure to only enable the `tracy-client` crate conditionally, via the
//! `enable` feature flag provided by this crate.
//!
//! [Tracy profiler]: https://github.com/wolfpld/tracy

use std::alloc;
#[doc(hidden)]
pub use tracy_client_sys as sys;

/// A handle representing a span of execution.
pub struct Span(sys::TracyCZoneCtx, std::marker::PhantomData<*mut sys::TracyCZoneCtx>);

impl Span {
    /// Start a new Tracy span.
    ///
    /// This function allocates the span information on the heap until it is read out by the
    /// profiler.
    ///
    /// `callstack_depth` specifies the maximum number of stack frames client should collect.
    pub fn new(name: &str, function: &str, file: &str, line: u32, callstack_depth: u16) -> Self {
        unsafe {
            sys::___tracy_init_thread();
            let loc = sys::___tracy_alloc_srcloc_name(
                line,
                file.as_ptr() as _,
                file.len(),
                function.as_ptr() as _,
                function.len(),
                name.as_ptr() as _,
                name.len(),
            );
            if callstack_depth == 0 {
                Self(sys::___tracy_emit_zone_begin_alloc(loc, 1), std::marker::PhantomData)
            } else {
                Self(sys::___tracy_emit_zone_begin_alloc_callstack(
                    loc,
                    adjust_stack_depth(callstack_depth).into(),
                    1,
                ), std::marker::PhantomData)
            }
        }
    }

    /// Emit a numeric value associated with this span.
    pub fn emit_value(&self, value: u64) {
        // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
        unsafe {
            sys::___tracy_emit_zone_value(self.0, value);
        }
    }

    /// Emit some text associated with this span.
    pub fn emit_text(&self, text: &str) {
        // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
        unsafe {
            sys::___tracy_emit_zone_text(self.0, text.as_ptr() as _, text.len());
        }
    }
}

impl Drop for Span {
    fn drop(&mut self) {
        // SAFE: the only way to construct `Span` is by creating a valid tracy zone context.
        unsafe {
            sys::___tracy_emit_zone_end(self.0);
        }
    }
}

/// A profiling wrapper around an allocator.
///
/// See documentation for [`std::alloc`](std::alloc) for more information about global allocators.
///
/// # Examples
///
/// In your executable, add:
///
/// ```rust
/// # use tracy_client::*;
/// #[global_allocator]
/// static GLOBAL: ProfiledAllocator<std::alloc::System> =
///     ProfiledAllocator::new(std::alloc::System, 100);
/// ```
pub struct ProfiledAllocator<T>(T, u16);

impl<T> ProfiledAllocator<T> {
    pub const fn new(inner_allocator: T, callstack_depth: u16) -> Self {
        Self(inner_allocator, adjust_stack_depth(callstack_depth))
    }

    fn emit_alloc(&self, ptr: *mut u8, size: usize) -> *mut u8 {
        unsafe {
            if self.1 == 0 {
                sys::___tracy_emit_memory_alloc(ptr as _, size);
            } else {
                sys::___tracy_emit_memory_alloc_callstack(ptr as _, size, self.1.into());
            }
        }
        ptr
    }

    fn emit_free(&self, ptr: *mut u8) -> *mut u8 {
        unsafe {
            if self.1 == 0 {
                sys::___tracy_emit_memory_free(ptr as _);
            } else {
                sys::___tracy_emit_memory_free_callstack(ptr as _, self.1.into());
            }
        }
        ptr
    }
}

unsafe impl<T: alloc::GlobalAlloc> alloc::GlobalAlloc for ProfiledAllocator<T> {
    unsafe fn alloc(&self, layout: alloc::Layout) -> *mut u8 {
        self.emit_alloc(self.0.alloc(layout), layout.size())
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: alloc::Layout) {
        self.0.dealloc(self.emit_free(ptr), layout)
    }

    unsafe fn alloc_zeroed(&self, layout: alloc::Layout) -> *mut u8 {
        self.emit_alloc(self.0.alloc_zeroed(layout), layout.size())
    }

    unsafe fn realloc(&self, ptr: *mut u8, layout: alloc::Layout, new_size: usize) -> *mut u8 {
        self.emit_alloc(
            self.0.realloc(self.emit_free(ptr), layout, new_size),
            new_size,
        )
    }
}

/// Indicate that rendering of a continuous frame has ended.
///
/// Typically should be inserted after a buffer swap.
///
/// In case you want to annotate secondary continuous frame sets, call the macro with a string
/// argument.
///
/// For non-continuous frame sets see [`Frame`](Frame).
///
/// # Examples
///
/// ```no_run
/// # use tracy_client::*;
/// # fn swap_buffers() {}
/// swap_buffers();
/// finish_continuous_frame!();
/// finish_continuous_frame!("some other frame loop");
/// ```
#[macro_export]
macro_rules! finish_continuous_frame {
    () => {
        unsafe {
            $crate::sys::___tracy_emit_frame_mark(std::ptr::null());
        }
    };
    ($name: literal) => {
        unsafe {
            $crate::sys::___tracy_emit_frame_mark(concat!($name, "\0").as_ptr() as _);
        }
    };
}

/// Start a non-continuous frame region.
#[macro_export]
macro_rules! start_noncontinuous_frame {
    ($name: literal) => {
        unsafe {
            let name = concat!($name, "\0");
            $crate::sys::___tracy_emit_frame_mark_start(name.as_ptr() as _);
            $crate::Frame::new_unchecked(name)
        }
    };
}

/// A non-continuous frame region.
///
/// Create with the [`start_noncontinuous_frame`](start_noncontinuous_frame) macro.
pub struct Frame(&'static str);

impl Frame {
    /// Use `start_noncontinuous_frame` instead.
    #[doc(hidden)]
    pub const unsafe fn new_unchecked(name: &'static str) -> Self {
        Self(name)
    }
}

impl Drop for Frame {
    fn drop(&mut self) {
        unsafe {
            sys::___tracy_emit_frame_mark_end(self.0.as_ptr() as _);
        }
    }
}

/// Output a message.
///
/// `callstack_depth` specifies the maximum number of stack frames client should collect.
pub fn message(message: &str, callstack_depth: u16) {
    unsafe {
        sys::___tracy_emit_message(
            message.as_ptr() as _,
            message.len(),
            adjust_stack_depth(callstack_depth).into()
        )
    }
}

/// Output a message with an associated color.
///
/// `callstack_depth` specifies the maximum number of stack frames client should collect.
///
/// The colour shall be provided as RGBA, where the least significant 8 bits represent the alpha
/// component and most significant 8 bits represent the red component.
pub fn color_message(message: &str, rgba: u32, callstack_depth: u16) {
    unsafe {
        sys::___tracy_emit_messageC(
            message.as_ptr() as _,
            message.len(),
            rgba >> 8,
            adjust_stack_depth(callstack_depth).into()
        )
    }
}

/// Create an instance of plot that can plot arbitrary `f64` values.
///
/// # Examples
///
/// ```
/// # use tracy_client::*;
/// static TEMPERATURE: Plot = create_plot!("temperature");
/// TEMPERATURE.point(37.0);
/// ```
#[macro_export]
macro_rules! create_plot {
    ($name: literal) => {
        unsafe { $crate::Plot::new_unchecked(concat!($name, "\0")) }
    };
}

/// A plot for plotting arbitary `f64` values.
///
/// Create with the [`create_plot`](create_plot) macro.
pub struct Plot(&'static str);

impl Plot {
    /// Use `create_plot!` instead.
    #[doc(hidden)]
    pub const unsafe fn new_unchecked(name: &'static str) -> Self {
        Self(name)
    }

    /// Add a point with `y`-axis value of `value` to the plot.
    pub fn point(&self, value: f64) {
        unsafe {
            sys::___tracy_emit_plot(self.0.as_ptr() as _, value);
        }
    }
}

/// Adjust the stack depth to maximum supported by tracy.
#[inline(always)]
const fn adjust_stack_depth(depth: u16) -> u16 {
    #[cfg(windows)]
    std::cmp::min(depth, 62)
    #[cfg(not(windows))]
    depth
}

#[cfg(test)]
mod tests {
    use super::*;

    #[global_allocator]
    static GLOBAL: ProfiledAllocator<alloc::System> = ProfiledAllocator::new(alloc::System, 100);

    #[test]
    fn zone_values() {
        let span = Span::new("test zone values", "zone_values", file!(), line!(), 100);
        span.emit_value(42);
        span.emit_text("some text");
    }

    #[test]
    fn finish_frameset() {
        for _ in 0..10 {
            finish_continuous_frame!();
        }
    }

    #[test]
    fn finish_secondary_frameset() {
        for _ in 0..5 {
            finish_continuous_frame!("every two seconds");
        }
    }

    #[test]
    fn non_continuous_frameset() {
        let _: Frame = start_noncontinuous_frame!("weird frameset");
    }

    #[test]
    fn plot_something() {
        static PLOT: Plot = create_plot!("a plot");
        for i in 0..10 {
            PLOT.point(i as f64);
        }
    }
}
