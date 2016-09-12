//! # Slog -  Structured, composable logging for Rust

#![cfg_attr(feature = "no_std", feature(alloc))]
#![cfg_attr(feature = "no_std", feature(collections))]
#![warn(missing_docs)]

#![no_std]

#[macro_use]
#[cfg(not(feature = "no_std"))]
extern crate std;
#[cfg(feature = "no_std")]
extern crate alloc;
#[cfg(feature = "no_std")]
extern crate collections;

use core::str::FromStr;
use core::fmt;
use core::result;

#[cfg(not(feature = "no_std"))]
use std::sync::Arc;
#[cfg(feature = "no_std")]
use alloc::arc::Arc;

#[cfg(not(feature = "no_std"))]
use std::slice;
#[cfg(feature = "no_std")]
use core::slice;

#[cfg(not(feature = "no_std"))]
use std::vec::Vec;
#[cfg(feature = "no_std")]
use collections::vec::Vec;

#[cfg(not(feature = "no_std"))]
use std::boxed::Box;
#[cfg(feature = "no_std")]
use alloc::boxed::Box;

/// Convenience function for building `&[OwnedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::discard();
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
/// }
/// ```
#[macro_export]
macro_rules! o(
    () => {
        vec![]
    };
    ($($k:expr => $v:expr),*) => {
        {
        use std;
        vec![$(($k, std::boxed::Box::new($v))),*]
        }
    };
);

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
///     let drain = slog::discard();
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "test info log"; "log-key" => true);
/// }
/// ```
///
/// Note that `"key" => value` part is optional.
///
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let drain = slog::discard();
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
///     let drain = slog::discard();
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
///     let drain = slog::discard();
///     let root = slog::Logger::root(drain, o!("key1" => "value1", "key2" => "value2"));
///     info!(root, "formatted: {}", 1);
/// }
/// ```

#[macro_export]
macro_rules! log(
    ($lvl:expr, $l:expr, $($k:expr => $v:expr),*; $($args:tt)+ ) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[$(($k, &$v)),*]))
        }
    };
    ($lvl:expr, $l:expr, $($args:tt),+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args),+), &[]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
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
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),*) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[$(($k, &$v)),*]))
        }
    };
);

/// Log message of a given level (alias)
///
/// Prefer shorter version, unless it clashes with
/// existing `log` crate macro.
#[macro_export]
macro_rules! slog_log(
    ($lvl:expr, $l:expr, $($k:expr => $v:expr),*; $($args:tt)+ ) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args)+), &[$(($k, &$v)),*]))
        }
    };
    ($lvl:expr, $l:expr, $($args:tt),+) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!($($args),+), &[]))
        }
    };
    ($lvl:expr, $l:expr, $msg:expr) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
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
    ($lvl:expr, $l:expr, $msg:expr; $($k:expr => $v:expr),*) => {
        if $lvl.as_usize() <= $crate::__slog_static_max_level().as_usize() {
            // prevent generating big `Record` over and over
            static RS : $crate::RecordStatic = $crate::RecordStatic {
                level: $lvl,
                file: file!(),
                line: line!(),
                column: column!(),
                function: "",
                module: module_path!(),
                target: module_path!(),
            };
            $l.log(&$crate::Record::new(&RS, format_args!("{}", $msg), &[$(($k, &$v)),*]))
        }
    };
);

/// Log critical level record
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
#[macro_export]
macro_rules! slog_crit(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Critical, $($args)+)
    };
);

/// Log error level record
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
#[macro_export]
macro_rules! slog_error(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Error, $($args)+)
    };
);


/// Log warning level record
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
#[macro_export]
macro_rules! slog_warn(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Warning, $($args)+)
    };
);

/// Log info level record
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
#[macro_export]
macro_rules! slog_info(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Info, $($args)+)
    };
);

/// Log debug level record
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
#[macro_export]
macro_rules! slog_debug(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Debug, $($args)+)
    };
);


/// Log trace level record
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
#[macro_export]
macro_rules! slog_trace(
    ($($args:tt)+) => {
        slog_log!($crate::Level::Trace, $($args)+)
    };
);

/// Serialization
pub mod ser;

pub use ser::{PushLazy, ValueSerializer, Serializer, Serialize};

include!("_level.rs");
include!("_logger.rs");
include!("_drain.rs");

/// Key value pair that can be part of a logging record
pub type BorrowedKeyValue<'a> = (&'static str, &'a ser::Serialize);

/// Key value pair that can be owned by `Logger`
///
/// See `o!(...)` macro.
pub type OwnedKeyValue = (&'static str, Box<ser::SyncSerialize>);

/// Chain of `OwnedKeyValue`-s of a `Logger` and its ancestors
pub struct OwnedKeyValueList {
    parent: Option<Arc<OwnedKeyValueList>>,
    values: Vec<OwnedKeyValue>,
}

impl OwnedKeyValueList {
    /// New `OwnedKeyValue` with a parent
    pub fn new(values: Vec<OwnedKeyValue>, parent: Arc<OwnedKeyValueList>) -> Self {
        OwnedKeyValueList {
            parent: Some(parent),
            values: values,
        }
    }

    /// New `OwnedKeyValue` without a parent (root)
    pub fn root(values: Vec<OwnedKeyValue>) -> Self {
        OwnedKeyValueList {
            parent: None,
            values: values,
        }
    }

    /// Get the parent element on the chain of values
    pub fn parent(&self) -> &Option<Arc<OwnedKeyValueList>> {
        &self.parent
    }

    /// Get the top-node values
    ///
    /// Since `OwnedKeyValueList` is just a chain of `Vec<OwnedKeyValue>`
    /// it's possible to iterate through it group-by-group.
    pub fn values(&self) -> &Vec<OwnedKeyValue> {
        &self.values
    }

    /// Iterator over `OwnedKeyValue`-s
    pub fn iter(&self) -> OwnedKeyValueListIterator {
        OwnedKeyValueListIterator::new(self)
    }
}

/// Iterator over `OwnedKeyValue`-s
pub struct OwnedKeyValueListIterator<'a> {
    next_node: &'a Option<Arc<OwnedKeyValueList>>,
    iter: slice::Iter<'a, OwnedKeyValue>,
}

impl<'a> OwnedKeyValueListIterator<'a> {
    fn new(node: &'a OwnedKeyValueList) -> Self {
        OwnedKeyValueListIterator {
            next_node: &node.parent,
            iter: node.values.iter(),
        }
    }
}

impl<'a> Iterator for OwnedKeyValueListIterator<'a> {
    type Item = &'a OwnedKeyValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => return Some(&*x),
                None => {
                    match *self.next_node {
                        Some(ref node) => {
                            self.iter = node.values.iter();
                            self.next_node = &node.parent;
                        }
                        None => return None,
                    }
                }
            }
        }
    }
}

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
    } else {
        FilterLevel::Trace
    }
}

#[cfg(test)]
mod tests;
