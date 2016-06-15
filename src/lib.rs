//! # Slog -  Structured, composable logging for Rust

extern crate crossbeam;
extern crate serde;
extern crate serde_json;

use std::sync::{Arc};
use crossbeam::sync::ArcCell;
use std::fmt;
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
    fn serialize(&self, key : &str, serializer : &mut Serializer);
}

pub trait Serializer {
    fn emit_u32(&mut self, key : &str, val : u32);
    fn emit_str(&mut self, key : &str, val : &str);
}

impl Serialize for u32 {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_u32(key, *self)
    }
}

impl Serialize for str {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self)
    }
}

impl<'a> Serialize for &'a str {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self)
    }
}

impl Serialize for String {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self.as_str())
    }
}

impl<S> Serializer for S where S : serde::Serializer {
    fn emit_u32(&mut self, key : &str, val : u32) {
        let _ = serde::Serialize::serialize(&val, self);
    }

    fn emit_str(&mut self, key : &str, val : &str) {
        let _ = serde::Serializer::serialize_map_elt(self, key, val);
    }
}


