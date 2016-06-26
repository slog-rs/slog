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

/// Convenience function for building `&[OwnedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let root = root_logger!(o!("key1" => "value1", "key2" => "value2"));
/// }
#[macro_export]
macro_rules! o(
    () => {
        &[]
    };
    ($($k:expr => $v:expr),*) => {
        {
        use std;
        &[$(($k, std::sync::Arc::new($v) as std::sync::Arc<$crate::ser::SyncSerialize>)),*]
        }
    };
);

/// Convenience function for building `&[BorrowedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
///
/// fn main() {
///     let root = root_logger!(o!());
///     let logger = root.new(b!("log-key" => true));
/// }
#[macro_export]
macro_rules! b(
    () => {
        &[]
    };
    ($($k:expr => $v:expr),*) => {
        &[$(($k, &$v)),*]
    };
);

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


