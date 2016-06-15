//! # Drains
//!
//! Drains are responsible for filtering, formatting and writing the log records
//! into given destination.
use super::{RecordInfo, Level, Serialize};
use std::{io, str};
use std::fmt::Write as FmtWrite;
use std::io::Write as IoWrite;
use std::sync::{Arc, Mutex};

use serde;
use serde_json;

///
/// Drain for Loggers
///
/// Implementing this trait allows writing own Drains
pub trait Drain : Send+Sync {
    // Return new RecordDrain to handle log record
    fn new_record(&self, info : &RecordInfo) -> Option<Box<RecordDrain>>;
}

/// Record Drain
///
/// Handles a single record sent to the drain
pub trait RecordDrain {
    /// Add a key:value to the record
    fn add(&mut self, key : &str, val : &Serialize);

    /// Finish handling the record.
    fn end(&mut self, &[(&str, &Serialize)]);
}


/// Drain formating records and writing them to a byte-stream (io::Write)
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer<W : io::Write> {
    io : Arc<Mutex<W>>,
}

impl<W : io::Write> Streamer<W> {
    pub fn new(io : W) -> Self {
        Streamer {
            io: Arc::new(Mutex::new(io)),
        }
    }
}

impl<W : 'static+io::Write+Send> Drain for Streamer<W> {
    fn new_record(&self, info : &RecordInfo) -> Option<Box<RecordDrain>> {
        Some(Box::new(RecordStreamer::new(self.io.clone(), info)))
    }
}


struct RecordStreamer<W : io::Write> {
    io : Arc<Mutex<W>>,
    serializer : Option<serde_json::Serializer<Vec<u8>>>,
}

impl<W : io::Write> RecordStreamer<W> {
    fn new(io : Arc<Mutex<W>>, info : &RecordInfo) -> Self {
        let mut serializer = serde_json::Serializer::new(vec!());

        /* TODO:
        write!(buf, "[{}][{:?}] {}",
               info.level,
               info.ts,
               info.msg).unwrap(); */

        RecordStreamer {
            io: io,
            serializer: Some(serializer),
        }
    }
}

struct KeyValueVisitor<'a> {
    values : &'a[(&'a str, &'a Serialize)],
    index : usize,
}

impl<'a> serde::ser::MapVisitor for KeyValueVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: serde::Serializer {
        if self.values.len() < self.index {

            let (key, val) = self.values[self.index];
            val.serialize(key, serializer);
            Ok(Some(()))
        } else {
            Ok(None)
        }
    }

    fn len(&self) -> Option<usize> {
        Some(self.values.len())
    }
}

impl<W : io::Write> RecordDrain for RecordStreamer<W> {
    fn add(&mut self, key : &str, val : &Serialize) {
    }

    fn end(&mut self, values : &[(&str, &Serialize)]) {
        for &(ref key, ref val) in values {
            val.serialize(key, self.serializer.as_mut().unwrap());
        }
        let mut io = self.io.lock().unwrap();
        let _ = write!(io, "{}", str::from_utf8(&self.serializer.take().unwrap().into_inner()).unwrap_or("INVALID UTF8 PRODUCED BY LOGGER"));
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
    fn new_record(&self, info : &RecordInfo) -> Option<Box<RecordDrain>> {
        if info.level.is_at_least(self.level) {
            return self.drain.new_record(info)
        } else {
            None
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
    fn new_record(&self, info : &RecordInfo) -> Option<Box<RecordDrain>> {
        match (self.drain1.new_record(info), self.drain2.new_record(info)) {
            (Some(r1), Some(r2)) => {
                Some(Box::new(DuplicateRecord::new(r1, r2)))
            },
            (Some(r1), None) => Some(r1),
            (None, Some(r2)) => Some(r2),
            (None, None) => None
        }
    }
}

struct DuplicateRecord {
    r1: Box<RecordDrain>,
    r2: Box<RecordDrain>,
}

impl DuplicateRecord {
    fn new(r1 : Box<RecordDrain>, r2 : Box<RecordDrain>) -> Self {
        DuplicateRecord{
            r1: r1,
            r2: r2,
        }
    }
}

impl RecordDrain for DuplicateRecord {
    fn add(&mut self, key : &str, val : &Serialize) {
        self.r1.add(key, val);
        self.r2.add(key, val);
    }

    fn end(&mut self, values : &[(&str, &Serialize)]) {
        self.r1.end(values);
        self.r2.end(values);
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
