//! # Drains
//!
//! Drains are responsible for filtering, formatting and writing the log records
//! into given destination.
use super::{ Level, Serialize};
use super::logger::RecordInfo;
use std::{io, str};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::sync::Mutex;

use serde;
use serde::Serializer;
use serde_json;

///
/// Drain for Loggers
///
/// Implementing this trait allows writing own Drains
pub trait Drain : Send+Sync {
    fn log(&self, info : &RecordInfo, &[(&str, &Serialize)]);
}

struct RecordVisitor<'a> {
    info : &'a RecordInfo,
    values : &'a[(&'a str, &'a Serialize)],
    index : usize,
}

impl<'a> serde::ser::MapVisitor for RecordVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: serde::Serializer {
        let ret = match self.index {
            0 => {self.info.level.as_str().serialize("level", serializer); Ok(Some(()))},
            1 => {format!("{:?}", self.info.ts).as_str().serialize("ts", serializer); Ok(Some(()))},
            2 => {self.info.msg.serialize("msg", serializer); Ok(Some(()))},
            _ => if self.values.len() < self.index - 3 {

                let (key, val) = self.values[self.index - 3];
                val.serialize(key, serializer);
                Ok(Some(()))
            } else {
                Ok(None)
            }
        };
        self.index += 1;
        ret
    }

    fn len(&self) -> Option<usize> {
        Some(self.values.len() + 3)
    }
}

/// Drain formating records and writing them to a byte-stream (io::Write)
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer<W : io::Write> {
    io : Mutex<W>,
}

impl<W : io::Write> Streamer<W> {
    pub fn new(io : W) -> Self {
        Streamer {
            io: Mutex::new(io),
        }
    }
}

impl<W : 'static+io::Write+Send> Drain for Streamer<W> {
    fn log(&self, info : &RecordInfo, values : &[(&str, &Serialize)]) {

        let mut serializer = serde_json::Serializer::new(vec!());

        let visitor = RecordVisitor {
            info : info,
            values : values,
            index : 0,
        };

        serializer.serialize_map(visitor);

        let mut io = self.io.lock().unwrap();
        let _ = write!(io, "{}", str::from_utf8(&serializer.into_inner()).unwrap_or("INVALID UTF8 PRODUCED BY LOGGER"));
    }
}


/// Record log level filter
///
/// Wraps a drain and passes records to it, only
/// if their level is at least given level.
pub struct FilterLevel<D : Drain> {
    level: Level,
    drain : D,
}

impl<D : Drain> FilterLevel<D> {
    /// Create FilterLevel wrapping given `subdrain` and passing to it records
    /// only of at least `level`.
    pub fn new(level : Level, subdrain : D) -> Self {
        FilterLevel {
            level: level,
            drain: subdrain,
        }
    }
}

impl<D : Drain> Drain for FilterLevel<D> {
    fn log(&self, info : &RecordInfo, values : &[(&str, &Serialize)]) {
        if info.level.is_at_least(self.level) {
            self.drain.log(info, values)
        }
    }
}


/// Duplicate records into two drains
pub struct Duplicate<D1 : Drain, D2 : Drain> {
    drain1 : D1,
    drain2 : D2,
}


impl<D1 : Drain, D2 : Drain> Duplicate<D1, D2> {
    /// Create FilterLevel wrapping given `subdrain` and passing to it records
    /// only of at least `level`.
    pub fn new(drain1 : D1, drain2 : D2) -> Self {
        Duplicate {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1 : Drain, D2 : Drain> Drain for Duplicate<D1, D2> {
    fn log(&self, info : &RecordInfo, values : &[(&str, &Serialize)]) {
        self.drain1.log(info, values);
        self.drain2.log(info, values);
    }
}

/// Create Streamer drain
pub fn stream<W : io::Write + Send>(io : W) -> Streamer<W> {
    Streamer::new(io)
}

/// Create FilterLevel drain
pub fn filter_level<D : Drain>(level : Level, d : D) -> FilterLevel<D> {
    FilterLevel::new(level, d)
}

/// Create Duplicate drain
pub fn duplicate<D1 : Drain, D2 : Drain>(d1 : D1, d2 : D2) -> Duplicate<D1, D2> {
    Duplicate::new(d1, d2)
}
