//! # Slog -  Structured, composable logging for Rust
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_json;
//! extern crate slog_term;
//!
//! use slog::*;
//! use slog::drain::{IntoLogger, AtomicSwitchCtrl};
//! use std::thread;
//!
//! use std::sync::atomic::Ordering::SeqCst;
//! use std::sync::atomic::AtomicUsize;
//! use std::sync::Arc;
//! use std::time::Duration;
//!
//! const VERSION : &'static str = "0.1.0";
//!
//! fn slow_fib(n : u64) -> u64 {
//!     match n {
//!         0|1|2 => 1,
//!         n => slow_fib(n-1) + slow_fib(n-2),
//!     }
//! }
//!
//!
//! fn main() {
//!     // Create a new drain hierarchy, for the need of your program.
//!     // Choose from collection of existing drains, or write your own `struct`
//!     // implementing `Drain` trait.
//!     let drain = slog_term::async_stderr();
//!
//!     // `AtomicSwitch` is a drain that wraps other drains and allows to change
//!     // it atomically in runtime
//!     let ctrl = AtomicSwitchCtrl::new(drain);
//!     let drain = ctrl.drain();
//!
//!     // Turn a drain into new group of loggers, sharing that drain.
//!     //
//!     // Note `o!` macro for more natural `OwnedKeyValue` sequence building.
//!     let root = drain.into_logger(o!("version" => VERSION, "build-id" => "8dfljdf"));
//!
//!     // Build logging context as data becomes available.
//!     //
//!     // Create child loggers from existing ones. Children clone `key: value`
//!     // pairs from their parents.
//!     let log = root.new(o!("child" => 1));
//!
//!     // Closures can be used for values that change at runtime.
//!     // Data captured by the closure needs to be `Send+Sync`.
//!     let counter = Arc::new(AtomicUsize::new(0));
//!     let log = log.new(o!("counter" => {
//!         let counter = counter.clone();
//!         // Note the `move` to capture `counter`,
//!         // and unfortunate `|_ : &_|` that helps
//!         // current `rustc` limitations. In the future,
//!         // a `|_|` could work.
//!         move |_ : &RecordInfo| { counter.load(SeqCst)}
//!     }));
//!
//!     // Note `b!` macro for more natural `BorrowedKeyValue` sequence building.
//!     log.info("before-fetch-add", b!()); // counter == 0
//!     counter.fetch_add(1, SeqCst);
//!     log.info("after-fetch-add", b!()); // counter == 1
//!
//!     // Drains can be swapped atomically (race-free).
//!     ctrl.set(
//!         // drains are composable
//!         drain::filter_level(
//!             Level::Info,
//!             drain::async_stream(
//!                 std::io::stderr(),
//!                 // multiple outputs formats are supported
//!                 slog_json::new(),
//!                 ),
//!             ),
//!         );
//!
//!     // Closures can be used for lazy evaluation:
//!     // This `slow_fib` won't be evaluated, as the current drain discards
//!     // "trace" level logging records.
//!     log.debug("debug", b!("lazy-closure" => |_ : &RecordInfo| slow_fib(40)));
//!
//!     // Loggers are internally atomically reference counted so can be cloned,
//!     // passed between threads and stored without hassle.
//!     let join = thread::spawn({
//!         let log = log.clone();
//!         move || {
//!             log.info("subthread", b!("stage" => "start"));
//!             thread::sleep(Duration::new(1, 0));
//!             log.info("subthread", b!("stage" => "end"));
//!         }
//!     });
//!
//!     join.join().unwrap();
//! }

#![warn(missing_docs)]

extern crate crossbeam;
extern crate chrono;
extern crate rustc_serialize as serialize;

#[macro_use]
extern crate error_chain;

use std::sync::Arc;
use std::fmt;

/// Convenience function for building `&[OwnedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use slog::drain::IntoLogger;
///
/// fn main() {
///     let drain = slog::drain::discard();
///     let root = drain.into_logger(o!("key1" => "value1", "key2" => "value2"));
/// }
#[macro_export]
macro_rules! o(
    () => {
        vec![]
    };
    ($($k:expr => $v:expr),*) => {
        {
        use std;
        vec![$(($k, std::sync::Arc::new($v) as std::sync::Arc<$crate::ser::SyncSerialize>)),*]
        }
    };
);

/// Convenience function for building `&[BorrowedKeyValue]`
///
/// ```
/// #[macro_use]
/// extern crate slog;
/// use slog::drain::IntoLogger;
///
/// fn main() {
///     let drain = slog::drain::discard();
///     let root = drain.into_logger(o!("key1" => "value1", "key2" => "value2"));
///     root.info("test info log", b!("log-key" => true));
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

/// Logger - logging handle
pub mod logger;

/// Serialization
pub mod ser;

/// Output formating
pub mod format;

pub use logger::Logger;
pub use logger::RecordInfo;

include!("_level.rs");

/// Key value pair that can be owned by `Logger`
pub type OwnedKeyValue = (&'static str, Arc<ser::SyncSerialize>);
/// Key value pair that can be part of each logging record
pub type BorrowedKeyValue<'a> = (&'static str, &'a ser::Serialize);

/// Values specific for this Logger and reference to it's parent values
#[derive(Clone)]
pub struct OwnedKeyValueNode {
    parent : Option<Arc<OwnedKeyValueNode>>,
    values : Vec<OwnedKeyValue>,
}

impl OwnedKeyValueNode {
    // New `OwnedKeyValue` with a parent
    pub fn new(values : Vec<OwnedKeyValue>, parent : Arc<OwnedKeyValueNode>) -> Self {
        OwnedKeyValueNode {
            parent : Some(parent),
            values : values,
        }
    }

    // New `OwnedKeyValue` without a parent (root)
    pub fn new_root(values : Vec<OwnedKeyValue>) -> Self {
        OwnedKeyValueNode {
            parent : None,
            values : values,
        }
    }

    /// Iterator over `OwnedKeyValue`-s
    pub fn iter(&self) -> OwnedKeyValueNodeIterator {
        OwnedKeyValueNodeIterator::new(self)
    }
}

/// Iterator over `OwnedKeyValue`-s
pub struct OwnedKeyValueNodeIterator<'a> {
    next_node : &'a Option<Arc<OwnedKeyValueNode>>,
    iter : std::slice::Iter<'a, OwnedKeyValue>,
}

impl<'a> OwnedKeyValueNodeIterator<'a> {
    fn new(node : &'a OwnedKeyValueNode) -> Self {
        OwnedKeyValueNodeIterator {
            next_node: &node.parent,
            iter: node.values.iter()
        }
    }
}

impl<'a> Iterator for OwnedKeyValueNodeIterator<'a> {
    type Item = &'a OwnedKeyValue;
    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.iter.next() {
                Some(x) => return Some(&*x),
                None => match self.next_node {
                    &Some(ref node) => {
                        self.iter = node.values.iter();
                        self.next_node = &node.parent;
                    }
                    &None => return None
                }
            }
        }
    }
}

#[cfg(test)]
mod tests;
