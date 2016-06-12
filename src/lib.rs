//! # Slog -  Structured, composable logging for Rust

extern crate crossbeam;

use std::sync::{Arc};
use crossbeam::sync::ArcCell;
use std::time;
use std::fmt;
use std::marker::PhantomData;
use std::io::Write as IoWrite;
use std::fmt::Write as FmtWrite;

/// Drains - logging outputs
pub mod drain;
use drain::*;

/// Logger - logging handle
pub mod logger;

pub use logger::Logger;

include!("_level.rs");

type DrainRef = Arc<ArcCell<Box<Drain>>>;
type KeyValue = (String, String);


trait Formatter {

}


/// Common information about a logging record
pub struct RecordInfo {
    /// Timestamp 
    pub ts : time::SystemTime,
    /// Logging level
    pub level : Level,
    /// Message
    pub msg : String,
}

/// Log record builder
pub struct RecordBuilder<'a> {
    record_drain: Option<Box<RecordDrain>>,
    phantom: PhantomData<&'a Logger>
}

impl<'a> RecordBuilder<'a> {
    pub fn add<'b, 'c, T : fmt::Display>(&'b mut self, key : &'b str, val : T) -> &'b mut Self {
        match self.record_drain {
            Some(ref mut drain) => drain.add(key, &val),
            None => {}
        }
        self
    }
}

impl<'a> Drop for RecordBuilder<'a> {
    fn drop(&mut self) {
        match self.record_drain.take() {
            Some(mut drain) => drain.end(),
            None => {}
        }
    }
}
