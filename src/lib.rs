//! # Slog -  Structured, composable logging for Rust
#![warn(missing_docs)]

extern crate crossbeam;
extern crate serde;
extern crate serde_json;
extern crate isatty;
extern crate ansi_term;
extern crate rustc_serialize as serialize;
extern crate chrono;

use std::sync::{Arc};
use crossbeam::sync::ArcCell;
use std::fmt;

/// Drains - logging outputs
pub mod drain;
use drain::*;

/// Logger - logging handle
pub mod logger;

/// Serialization
pub mod ser;

/// Output formating
pub mod format;

pub use logger::Logger;

include!("_level.rs");

type DrainRef = Arc<ArcCell<Box<Drain>>>;

/// Key value pair that can be owned by `Logger`
pub type OwnedKeyValue = (&'static str, Arc<ser::SyncSerialize>);
/// Key value pair that can be part of each logging record
pub type BorrowedKeyValue<'a> = (&'static str, &'a ser::Serialize);


/// Log a custom level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let root = root_logger!("key" => "value");
///     log!(root, slog::Level::Debug, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! log(
    ($logger:ident, $lvl:expr, $msg:expr) => {
        $logger.log($lvl, $msg, &[])
    };
    ($logger:ident, $lvl:expr, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($lvl, $msg, &[$(($k, &$v)),*])
    };
);

/// Log a critical level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     critical!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! critical(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Critical, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Critical, $msg, &[$(($k, &$v)),*])
    };
);

/// Log an error level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     error!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! error(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Error, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Error, $msg, &[$(($k, &$v)),*])
    };
);


/// Log a warning level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     warn!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! warn(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Warning, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Warning, $msg, &[$(($k, &$v)),*])
    };
);


/// Log an info level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     info!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! info(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Info, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Info, $msg, &[$(($k, &$v)),*])
    };
);


/// Log a debug level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     debug!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! debug(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Debug, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Debug, $msg, &[$(($k, &$v)),*])
    };
);

/// Log a trace level logging record
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
///     trace!(root, "something happened", "counter" => 1);
/// }
/// ```
#[macro_export]
macro_rules! trace(
    ($logger:ident, $msg:expr) => {
        $logger.log($crate::Level::Trace, $msg, &[])
    };
    ($logger:ident, $msg : expr, $($k:expr => $v:expr),*) => {
        $logger.log($crate::Level::Trace, $msg, &[$(($k, &$v)),*])
    };
);

/// Build a root logger
///
/// All children and their children and so on form one hierarchy
/// sharing a common drain.
///
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// fn main() {
///     let root = root_logger!("key" => "value");
/// }
/// ```
#[macro_export]
macro_rules! root_logger(
    () => {
        $crate::Logger::new_root(vec!())
    };
    ($($k:expr => $v:expr),*) => {
        $crate::Logger::new_root(vec!($(($k, std::sync::Arc::new($v))),*))
    };
);

/// Build a child logger
///
/// Child logger copies all existing values from the parent.
///
/// All children and their children and so on form one hierarchy sharing
///
/// a common drain.
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let root = root_logger!("key" => "value");
///     let log = child_logger!(root, "key1" => 3);
/// }
/// ```
#[macro_export]
macro_rules! child_logger(
    ($parent:expr) => {
        $parent.new(vec!())
    };
    ($parent:expr, $($k:expr => $v:expr),*) => {
        $parent.new(vec!($(($k, std::sync::Arc::new($v))),*))
    };
);
