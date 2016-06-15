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
use super::{DrainRef, KeyValue, RecordBuilder, Level, RecordInfo};
use std::marker::PhantomData;
use std::sync::Arc;
use crossbeam::sync::ArcCell;
use std::{time, fmt, io};

use drain;

struct LoggerInner {
    drain: DrainRef,
    values: Vec<KeyValue>,
}


#[derive(Clone)]
pub struct Logger {
    inner: Arc<LoggerInner>
}

impl Logger {
    /// Build a root logger
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    pub fn root() -> LoggerBuilder<'static> {
        LoggerBuilder {
            drain: Arc::new(ArcCell::new(Arc::new(Box::new(drain::Streamer::new(io::stderr()))))),
            values: vec!(),
            phantom : PhantomData::default(),
        }
    }

    /// Build a child logger
    ///
    /// Child logger copies all existing values from the parent.
    ///
    /// All children and their children and so on form one hierarchy sharing
    /// a common drain.
    pub fn new<'a>(&'a self) -> LoggerBuilder<'a> {
        LoggerBuilder {
            drain: self.inner.drain.clone(),
            values: self.inner.values.clone(),
            phantom : PhantomData::default(),
        }
    }

    /// Build a root logger copying values
    ///
    /// This logger will copy all values from it's parent,
    /// but won't share drain with it, forming a new hierarchy.
    pub fn new_root<'a>(&'a self) -> LoggerBuilder<'a> {
        LoggerBuilder {
            drain: Arc::new(ArcCell::new(Arc::new(Box::new(drain::Streamer::new(io::stderr()))))),
            values: self.inner.values.clone(),
            phantom : PhantomData::default(),
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

    /// Log a critical level log record
    pub fn critical<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Critical, msg)
    }

    /// Log an error level log record
    pub fn error<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Error, msg)
    }

    /// Log a warning level log record
    pub fn warning<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Warning, msg)
    }

    /// Log an info level log record
    pub fn info<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Info, msg)
    }

    /// Log a debug level log record
    pub fn debug<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Debug, msg)
    }

    /// Log a trace level log record
    pub fn trace<'a, 'b>(&'a self, msg : &'b str) -> RecordBuilder<'a> {
        self.log(Level::Trace, msg)
    }

    /// Log a record with a given logging level
    pub fn log<'a, 'b>(&'a self, lvl : Level, msg : &'b str) -> RecordBuilder<'a> {

        let drain = self.inner.drain.get();


        let info = RecordInfo {
            ts: time::SystemTime::now(),
            msg: msg.to_string(),
            level: lvl,
        };

        let record_drain = drain.new_record(&info);

        // TODO: check the drain logging level here to skip logging
        // altogether?
        let mut builder = RecordBuilder {
            record_drain: record_drain,
            phantom: PhantomData::default(),
        };

        for &(ref k, ref v) in &self.inner.values {
            builder.add(&k, v as &str);
        }

        builder
    }
}

/// Logger builder
///
/// Temporary object used as a handle to add key-value pairs
/// to new logger using `add(...)`.
///
/// Use `end()` method to receive the Logger.
pub struct LoggerBuilder<'a> {
    drain: DrainRef,
    values: Vec<KeyValue>,
    phantom: PhantomData<&'a LoggerInner>,
}
impl<'a> LoggerBuilder<'a> {
    /// Add key-value pair
    pub fn add<T : fmt::Display>(mut self, key : &str, val : T) -> Self {
        self.values.push((key.to_owned(), format!("{}", val)));
        self
    }

    /// Finish building the logger
    pub fn end(self) -> Logger {
        Logger {
            inner: Arc::new(
               LoggerInner {
                   values: self.values,
                   drain: self.drain,
               }
           )
        }
    }
}
