use client::Client;
use tracing_subscriber::fmt::format::DefaultFields;
use tracing_subscriber::fmt::FormatFields;

/// Configuration of the [`TracyLayer`] behaviour.
///
/// For most users [`DynamicConfig`] is going to be a good default option, however advanced users
/// can implement this trait manually to achieve better performance through constant evaluation,
/// to override the formatter used or to otherwise modify the behaviour of `TracyLayer` in ways
/// that are not exposed via the `DynamicConfig` type.
///
/// # Examples
///
/// ## Implementation with compile-time configuration
///
/// ```
/// #[derive(Default)]
/// struct ConstantTracyConfig {
///     formatter: tracing_subscriber::fmt::format::DefaultFields,
/// }
///
/// impl tracing_tracy::Config for ConstantTracyConfig {
///     type Formatter = tracing_subscriber::fmt::format::DefaultFields;
///     fn formatter(&self) -> &Self::Formatter { &self.formatter }
///     fn stack_depth(&self) -> u16 { 0 } // Same as the default trait impl.
///     fn format_fields_in_zone_name(&self) -> bool { true } // Same as the default trait impl.
/// }
/// ```
///
/// With this sort of setup the compiler will be able to inline calls to `stack_depth` and
/// `format_fields_in_zone_name` and optimize accordingly.
pub trait Config {
    type Formatter: for<'writer> FormatFields<'writer> + 'static;

    /// Use a custom field formatting implementation.
    fn formatter(&self) -> &Self::Formatter;

    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Note that enabling callstack collection can and will introduce a non-trivial overhead at
    /// every instrumentation point. Specifying 0 frames will disable stack trace collection.
    ///
    /// Default implementation returns `0`.
    fn stack_depth(&self) -> u16 {
        0
    }

    /// Specify whether or not to include tracing span fields in the tracy zone name, or to emit
    /// them as zone text.
    ///
    /// The former enables zone analysis along unique span field invocations, while the latter
    /// aggregates every invocation of a given span into a single zone, irrespective of field
    /// values.
    ///
    /// Default implementation returns `true`.
    fn format_fields_in_zone_name(&self) -> bool {
        true
    }
}

/// A type that implements the [`Config`] trait with runtime-adjustable values.
///
/// Ues the [`tracing_subscriber`] [`DefaultFields`] formatter. If not appropriate, consider
/// implementing the `Config` trait yourself.
pub struct DynamicConfig {
    fmt: DefaultFields,
    stack_depth: u16,
    fields_in_zone_name: bool,
}

impl DynamicConfig {
    /// Create a new implementation of `Config` that permits non-constant configuration.
    #[must_use]
    pub fn new() -> Self {
        DynamicConfig {
            fmt: DefaultFields::new(),
            stack_depth: 0,
            fields_in_zone_name: true,
        }
    }

    /// Specify the maximum number of stack frames that will be collected.
    ///
    /// Note that enabling callstack collection can and will introduce a non-trivial overhead at
    /// every instrumentation point. Specifying 0 frames will disable stack trace collection.
    ///
    /// Defaults to `0`.
    #[must_use]
    pub const fn with_stack_depth(mut self, stack_depth: u16) -> Self {
        self.stack_depth = stack_depth;
        self
    }

    /// Specify whether or not to include tracing span fields in the tracy zone name, or to emit
    /// them as zone text.
    ///
    /// The former enables zone analysis along unique span field invocations, while the latter
    /// aggregates every invocation of a given span into a single zone, irrespective of field
    /// values.
    ///
    /// Defaults to `true`.
    #[must_use]
    pub const fn with_fields_in_zone_name(mut self, fields_in_zone_name: bool) -> Self {
        self.fields_in_zone_name = fields_in_zone_name;
        self
    }
}

impl Config for DynamicConfig {
    type Formatter = DefaultFields;

    fn formatter(&self) -> &Self::Formatter {
        &self.fmt
    }

    fn stack_depth(&self) -> u16 {
        self.stack_depth
    }

    fn format_fields_in_zone_name(&self) -> bool {
        self.fields_in_zone_name
    }
}
