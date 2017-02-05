/// Convenience macro for building `SyncMultiSerialize` trait object
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let _root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
/// }
/// ```
#[cfg(feature = "std")]
#[macro_export]
macro_rules! o(
    (@ $args_ready:expr; $k:expr => $v:expr) => {
        o!(@ (($k, $v), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => $v:expr, $($args:tt)* ) => {
        o!(@ (($k, $v), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; ) => {
        $args_ready
    };
    (@ $args_ready:expr;, ) => {
        $args_ready
    };
    ($($args:tt)*) => {
        $crate::OwnedKVGroup(::std::boxed::Box::new(o!(@ (); $($args)*)))
    };
);

/// Convenience macro for building `SyncMultiSerialize` trait object
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let _root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
/// }
/// ```
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! o(
    (@ $args_ready:expr; $k:expr => $v:expr) => {
        o!(@ (($k, $v), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => $v:expr, $($args:tt)* ) => {
        o!(@ (($k, $v), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; ) => {
        $args_ready
    };
    (@ $args_ready:expr;, ) => {
        $args_ready
    };
    ($($args:tt)*) => {
        $crate::OwnedKVGroup(::alloc::Box::new(o!(@ (); $($args)*)))
    };
);

/// An alternative, longer-name version of `o` macro
///
/// Use in case of macro name collisions
#[cfg(feature = "std")]
#[macro_export]
macro_rules! slog_o(
    (@ $args_ready:expr; $k:expr => $v:expr) => {
        o!(@ ($k, $v, $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => $v:expr, $($args:tt)* ) => {
        o!(@ ($k, $v, $args_ready); $($args)* )
    };
    (@ $args_ready:expr; ) => {
        $args_ready
    };
    (@ $args_ready:expr;, ) => {
        $args_ready
    };
    ($($args:tt)*) => {
        $crate::OwnedKVGroup(::std::boxed::Box::new(o!(@ (); $($args)*)))
    };
);

/// An alternative, longer-name version of `o` macro
///
/// Use in case of macro name collisions
#[cfg(not(feature = "std"))]
#[macro_export]
macro_rules! slog_o(
    (@ $args_ready:expr; $k:expr => $v:expr) => {
        o!(@ (($k, $v), $args_ready); )
    };
    (@ $args_ready:expr; $k:expr => $v:expr, $($args:tt)* ) => {
        o!(@ (($k, $v), $args_ready); $($args)* )
    };
    (@ $args_ready:expr; ) => {
        $args_ready
    };
    (@ $args_ready:expr;, ) => {
        $args_ready
    };
    ($($args:tt)*) => {
        $crate::OwnedKVGroup(::alloc::Box::new(o!(@ (); $($args)*)))
    };
);

#[allow(unknown_lints)]
#[allow(inline_always)]
#[inline(always)]
#[doc(hidden)]
/// Not an API
pub fn __slog_static_max_level() -> FilterLevel {
    if !cfg!(debug_assertions) {
        if cfg!(feature = "release_max_level_off") {
            return FilterLevel::Off;
        } else if cfg!(feature = "release_max_level_error") {
            return FilterLevel::Error;
        } else if cfg!(feature = "release_max_level_warn") {
            return FilterLevel::Warning;
        } else if cfg!(feature = "release_max_level_info") {
            return FilterLevel::Info;
        } else if cfg!(feature = "release_max_level_debug") {
            return FilterLevel::Debug;
        } else if cfg!(feature = "release_max_level_trace") {
            return FilterLevel::Trace;
        }
    }
    if cfg!(feature = "max_level_off") {
        FilterLevel::Off
    } else if cfg!(feature = "max_level_error") {
        FilterLevel::Error
    } else if cfg!(feature = "max_level_warn") {
        FilterLevel::Warning
    } else if cfg!(feature = "max_level_info") {
        FilterLevel::Info
    } else if cfg!(feature = "max_level_debug") {
        FilterLevel::Debug
    } else if cfg!(feature = "max_level_trace") {
        FilterLevel::Trace
    } else {
        if !cfg!(debug_assertions) {
            FilterLevel::Info
        } else {
            FilterLevel::Debug
        }
    }
}



/// Log message of a given level
///
/// Use wrappers `error!`, `warn!` etc. instead
///
/// The `max_level_*` features can be used to statically disable logging at
/// various levels.
///
/// Use longer name version macros if you want to prevent clash with legacy `log`
/// crate macro names (see `examples/alternative_names.rs`).
///
/// The following invocations are supported.
///
/// Simple:
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "test info log"; "log-key" => true);
/// }
/// ```
///
/// Note that `"key" => value` part is optional.
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "test info log");
/// }
/// ```
///
/// Formatting support:
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "log-key" => true; "formatted: {}", 1);
/// }
/// ```
///
/// Again, `"key" => value` is optional.
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::Discard;
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "formatted: {}", 1);
/// }
/// ```
#[macro_export]
macro_rules! log(
    ($lvl:expr, $l:expr, $($k:expr => $v:expr),+; $($args:tt)+ ) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[$(($k, &$v)),+]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[$(($k, &$v)),+]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),+,) => {
        log!($lvl, $l, $msg; $($k => $v),+)
    };
    ($lvl:expr, $l:expr, $($args:tt)+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[]))
        }
    };
);

/// Log message of a given level (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `log` for format documentation.
///
/// ```
/// #[macro_use(o,slog_log,slog_info)]
/// extern crate slog;
///
/// fn main() {
///     let log = slog::Logger::root(slog::Discard, o!());
///
///     slog_info!(log, "some interesting info"; "where" => "right here");
/// }
/// ```
#[macro_export]
macro_rules! slog_log(
    ($lvl:expr, $l:expr, $($k:expr => $v:expr),+; $($args:tt)+ ) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[$(($k, &$v)),+]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[$(($k, &$v)),+]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),+,) => {
        log!($lvl, $l, $msg; $($k => $v),+)
    };
    ($lvl:expr, $l:expr, $($args:tt)+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic<'static> = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[]))
        }
    };
);

/// Log critical level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! crit(
    ($($args:tt)+) => {
        log!($crate::Level::Critical, $($args)+)
    };
);

/// Log critical level record (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_crit(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Critical, $($args)+)
    };
);

/// Log error level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! error(
    ($($args:tt)+) => {
        log!($crate::Level::Error, $($args)+)
    };
);

/// Log error level record
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_error(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Error, $($args)+)
    };
);


/// Log warning level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! warn(
    ($($args:tt)+) => {
        log!($crate::Level::Warning, $($args)+)
    };
);

/// Log warning level record (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_warn(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Warning, $($args)+)
    };
);

/// Log info level record
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! info(
    ($($args:tt)+) => {
        log!($crate::Level::Info, $($args)+)
    };
);

/// Log info level record (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_info(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Info, $($args)+)
    };
);

/// Log debug level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! debug(
    ($($args:tt)+) => {
        log!($crate::Level::Debug, $($args)+)
    };
);

/// Log debug level record (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_debug(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Debug, $($args)+)
    };
);


/// Log trace level record
///
/// See `log` for documentation.
#[macro_export]
macro_rules! trace(
    ($($args:tt)+) => {
        log!($crate::Level::Trace, $($args)+)
    };
);

/// Log trace level record (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
///
/// See `slog_log` for documentation.
#[macro_export]
macro_rules! slog_trace(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Trace, $($args)+)
    };
);

