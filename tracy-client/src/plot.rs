use crate::Client;

/// Name of a plot.
///
/// Create with the [`plot_name!`](crate::plot_name) macro.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlotName(pub(crate) &'static str);

/// Instrumentation for drawing 2D plots.
impl Client {
    /// Add a point with an y-axis value of `value` to the plot named `plot_name`.
    ///
    /// # Examples
    ///
    /// ```
    /// # let client = tracy_client::Client::start();
    /// tracy_client::Client::running()
    ///     .expect("client must be running")
    ///     .plot(tracy_client::plot_name!("temperature"), 37.0);
    /// ```
    pub fn plot(&self, plot_name: PlotName, value: f64) {
        #[cfg(feature = "enable")]
        unsafe {
            // SAFE: We made sure the `plot` refers to a null-terminated string.
            sys::___tracy_emit_plot(plot_name.0.as_ptr().cast(), value);
        }
    }
}

/// Construct a [`PlotName`].
///
/// The resulting value may be used as an argument for the [`Client::plot`] method. The macro can
/// be used in a `const` context.
#[macro_export]
macro_rules! plot_name {
    ($name: expr) => {
        unsafe { $crate::internal::create_plot(concat!($name, "\0")) }
    };
}

/// Convenience macro for [`Client::plot`] on the current client.
///
/// # Panics
///
/// - If a `Client` isn't currently running.
#[macro_export]
macro_rules! plot {
    ($name: expr, $value: expr) => {{
        $crate::Client::running()
            .expect("plot! without a running Client")
            .plot($crate::plot_name!($name), $value)
    }};
}
