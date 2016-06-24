//! # Drains
//!
//! Drains are responsible for filtering, formatting and writing the log records
//! into given destination.
use super::{Level};
use super::format;
use super::logger::RecordInfo;
use std::{io, str};
use std::sync::Mutex;
use super::{OwnedKeyValue,BorrowedKeyValue};

///
/// Drain for Loggers
///
/// Implementing this trait allows writing own Drains
pub trait Drain : Send+Sync {
    /// Write one logging record
    fn log(&self, info : &RecordInfo, &[OwnedKeyValue], &[BorrowedKeyValue]);
}


/// Drain formating records and writing them to a byte-stream (io::Write)
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer<W : io::Write, F : format::Format> {
    io : Mutex<W>,
    format : F,
}

impl<W : io::Write, F : format::Format> Streamer<W, F> {
    /// Create new `Streamer` writing to `io` using `format`
    pub fn new(io : W, format : F) -> Self {
        Streamer {
            io: Mutex::new(io),
            format : format,
        }
    }
}

impl<W : 'static+io::Write+Send, F : format::Format+Send> Drain for Streamer<W, F> {
    fn log(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], values : &[BorrowedKeyValue]) {
        let formatted = self.format.format(info, logger_values, values);

        let mut io = self.io.lock().unwrap();
        let _ = write!(io, "{}", formatted);
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
    fn log(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], values : &[BorrowedKeyValue]) {
        if info.level.is_at_least(self.level) {
            self.drain.log(info, logger_values, values)
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
    fn log(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], values : &[BorrowedKeyValue]) {
        self.drain1.log(info, logger_values, values);
        self.drain2.log(info, logger_values, values);
    }
}

/// Create Streamer drain
pub fn stream<W : io::Write + Send, F : format::Format>(io : W, format : F) -> Streamer<W, F> {
    Streamer::new(io, format)
}

/// Create FilterLevel drain
pub fn filter_level<D : Drain>(level : Level, d : D) -> FilterLevel<D> {
    FilterLevel::new(level, d)
}

/// Create Duplicate drain
pub fn duplicate<D1 : Drain, D2 : Drain>(d1 : D1, d2 : D2) -> Duplicate<D1, D2> {
    Duplicate::new(d1, d2)
}
