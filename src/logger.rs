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
use super::{DrainRef, OwnedKeyValue, Level, BorrowedKeyValue};
use std::sync::Arc;
use crossbeam::sync::ArcCell;
use std::{io};
use super::format;

use isatty::stderr_isatty;
use drain;

use chrono;

struct LoggerInner {
    drain: DrainRef,
    values: Vec<OwnedKeyValue>,
}


#[derive(Clone)]
/// Logger
pub struct Logger {
    inner: Arc<LoggerInner>,
}

impl Logger {
    /// Build a root logger
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    ///
    /// Use `root_logger!` macro instead.
    #[doc(hidden)]
    pub fn new_root(values : Vec<OwnedKeyValue>) -> Logger {
        let drain = Arc::new(
            ArcCell::new(
                Arc::new(
                    Box::new(
                        drain::Streamer::new(
                            io::stderr(),
                            if stderr_isatty() {
                                format::Terminal::colored()
                            } else {
                                format::Terminal::plain()
                            }
                            )
                        ) as Box<drain::Drain>
                    )
                )
            );
        Logger{
            inner: Arc::new(LoggerInner {
                drain: drain,
                values: values,
            }),
        }
    }

    /// Build a child logger
    ///
    /// Child logger copies all existing values from the parent.
    ///
    /// All children and their children and so on form one hierarchy sharing
    /// a common drain.
    ///
    /// Use `child_logger!` macro instead.
    #[doc(hidden)]
    pub fn new(&self, values : Vec<OwnedKeyValue>) -> Logger {
        let mut new_values = self.inner.values.clone();
        new_values.extend_from_slice(&values);
        Logger{
            inner: Arc::new(LoggerInner {
                drain: self.inner.drain.clone(),
                values: new_values,
            }),
        }
    }

    /// Set the drain for logger and it's hierarchy
    pub fn set_drain<D : drain::Drain>(&self, drain : D) {
        let _ = self.inner.drain.set(Arc::new(Box::new(drain)));
    }

    /// Swap the existing drain with a new one
    ///
    /// As the drains are shared between threads, and might still be
    /// referenced `Arc`s are being used to reference-count them.
    pub fn swap_drain(&self, drain : Arc<Box<drain::Drain>>) -> Arc<Box<drain::Drain>> {
        self.inner.drain.set(drain)
    }

    /// Log one logging record
    ///
    /// Use specific logging macros instead.
    #[doc(hidden)]
    pub fn log<'a>(&'a self, lvl : Level, msg : &'a str, values : &'a[BorrowedKeyValue<'a>]) {

        let info = RecordInfo {
            ts: chrono::UTC::now(),
            msg: msg.to_string(),
            level: lvl,
        };

        self.inner.drain.get().log(&info, self.inner.values.as_slice(), values);
    }
}

/// Common information about a logging record
pub struct RecordInfo {
    /// Timestamp
    pub ts : chrono::DateTime<chrono::UTC>,
    /// Logging level
    pub level : Level,
    /// Message
    pub msg : String,
}
