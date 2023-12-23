use crate::Client;

/// Name of a plot.
///
/// Create with the [`plot_name!`](crate::plot_name) macro.
#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct PlotName(pub(crate) &'static str);

impl PlotName {
    /// Construct a `PlotName` dynamically, leaking the provided String.
    ///
    /// You should call this function once for a given name, and store the returned `PlotName` for
    /// continued use, to avoid rapid memory use growth. Whenever possible, prefer the
    /// [`plot_name!`](crate::plot_name) macro, which takes a literal name and doesn't leak memory.
    ///
    /// The resulting value may be used as an argument for the the [`Client::secondary_frame_mark`]
    /// and [`Client::non_continuous_frame`] methods.
    pub fn new_leak(name: String) -> Self {
        // Ensure the name is null-terminated.
        let mut name = name;
        name.push('\0');
        // Drop excess capacity by converting into a boxed str, then leak.
        let name = Box::leak(name.into_boxed_str());
        Self(name)
    }
}

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
