//! # Logger
//!
//! Loggers are thread-safe and reference counted, so can be freely
//! passed around the code.
//!
//! Each logger is built with a set of key-values.
//!
//! Child loggers are build from existing loggers, and copy
//! all the key-values from their parents
//!
//! Loggers form hierarchies sharing a drain. Setting a drain on
//! any logger will change it on all loggers in given hierarchy.
use super::{OwnedKeyValue, Level, BorrowedKeyValue};
use std::sync::Arc;
use std::cell::RefCell;
use crossbeam::sync::ArcCell;

use drain;

use chrono;

// TODO: Implement custom clone, that starts with a new buffer
#[derive(Clone)]
/// Logger
pub struct Logger {
    drain: Arc<ArcCell<Box<drain::Drain>>>,
    values: Vec<OwnedKeyValue>,
    buf : RefCell<Vec<u8>>,
}

impl Logger {
    /// Build a root logger
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    ///
    /// Use `o!` macro to help build `values`
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::new_root(o!("key1" => "value1", "key2" => "value2"));
    /// }
    pub fn new_root(values: &[OwnedKeyValue]) -> Logger {
        let drain =
            Arc::new(ArcCell::new(Arc::new(Box::new(drain::discard()) as Box<drain::Drain>)));
        Logger {
            drain: drain,
            values: values.to_vec(),
            buf : RefCell::new(Vec::with_capacity(128)),
        }
    }

    /// Build a child logger
    ///
    /// Child logger copies all existing values from the parent.
    ///
    /// All children, their children and so on, form one hierarchy sharing
    /// a common drain.
    ///
    /// Use `o!` macro to help build `values`
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::new_root(o!("key1" => "value1", "key2" => "value2"));
    ///     let log = root.new(o!("key" => "value"));
    /// }

    pub fn new(&self, values: &[OwnedKeyValue]) -> Logger {
        let mut new_values = self.values.clone();
        new_values.extend_from_slice(values);
        Logger {
            drain: self.drain.clone(),
            values: new_values,
            buf : RefCell::new(Vec::with_capacity(128)),
        }
    }

    /// Set the drain for logger and it's hierarchy
    pub fn set_drain<D: drain::Drain>(&self, drain: D) {
        let _ = self.drain.set(Arc::new(Box::new(drain)));
    }

    /// Swap the existing drain with a new one
    ///
    /// As the drains are shared between threads, and might still be
    /// referenced `Arc`s are being used to reference-count them.
    pub fn swap_drain(&self, drain: Arc<Box<drain::Drain>>) -> Arc<Box<drain::Drain>> {
        self.drain.set(drain)
    }

    /// Log one logging record
    ///
    /// Use specific logging functions instead.
    pub fn log<'a>(&'a self, lvl: Level, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {

        let info = RecordInfo {
            ts: chrono::UTC::now(),
            msg: msg.to_string(),
            level: lvl,
        };

        // By default errors in loggers are ignored
        let mut buf = self.buf.borrow_mut();
        let _ = self.drain.get().log(&mut *buf, &info, self.values.as_slice(), values);
        // TODO: Double check if this will not zero the old bytes
        buf.clear();
    }

    /// Log critical level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn critical<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Critical, msg, values);
    }

    /// Log error level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn error<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Error, msg, values);
    }

    /// Log warning level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn warn<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Warning, msg, values);
    }

    /// Log info level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn info<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Info, msg, values);
    }

    /// Log debug level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn debug<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Debug, msg, values);
    }

    /// Log trace level record
    ///
    /// Use `b!` macro to help build `values`
    pub fn trace<'a>(&'a self, msg: &'a str, values: &'a [BorrowedKeyValue<'a>]) {
        self.log(Level::Trace, msg, values);
    }
}

/// Common information about a logging record
pub struct RecordInfo {
    /// Timestamp
    pub ts: chrono::DateTime<chrono::UTC>,
    /// Logging level
    pub level: Level,
    /// Message
    pub msg: String,
}

impl RecordInfo {
    /// Create a new `RecordInfo` with a current timestamp
    pub fn new(level: Level, msg: String) -> Self {
        RecordInfo {
            ts: chrono::UTC::now(),
            level: level,
            msg: msg,
        }
    }
}
