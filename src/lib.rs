//! # Slog -  Structured, composable logging for Rust

extern crate crossbeam;
extern crate serde;
extern crate serde_json;

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

pub trait Serialize {
    fn serialize_serde_json(&self, serializer : &mut serde_json::Serializer);
}

impl<S> Serialize for S where S : serde::Serialize {
    fn serialize_serde_json(&self, serializer : &mut serde_json::Serializer) {
//        serializer.serialize(self);
        serde::Serialize::serialize(self, serializer)
    }
}

pub trait Serializer {
    fn serialize_serde_json(&mut self, val : &Serialize);
}

impl Serializer for serde_json::Serializer {
    fn serialize(&mut self, val : &Serialize) {
        serde::Serialize::serialize_serde(val, self);
    }
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
    pub fn add<'b, 'c, T : Serialize>(&'b mut self, key : &'b str, val : T) -> &'b mut Self {
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
