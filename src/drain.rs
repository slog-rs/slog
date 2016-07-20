//! # Drains
//!
//! Drains are responsible for filtering, formatting and writing the log records
//! into given destination.
use std::io;
use std::sync::Mutex;
use std::sync::mpsc;
use std::thread;

use super::Level;
use super::format;
use super::logger::RecordInfo;
use super::{OwnedKeyValue, BorrowedKeyValue};

#[allow(missing_docs)]
mod error {
    use super::super::format;
    use std::io;

    error_chain! {
        types {
            Error, ErrorKind, ChainErr, Result;
        }
        links {
            format::Error, format::ErrorKind, Format;
        }
        foreign_links {
            io::Error, Io, "io error";
        }
        errors {
            SendError {
                description("sending to another thread failed")
                    display("sending to another thread failed")
            }
            LockError {
                description("locking mutex failed")
                    display("locking mutex failed")
            }
            FailoverExhausted {
                description("failover drains exhausted")
                    display("failover drains exhausted")
            }

        }
    }
}

pub use self::error::{Error, Result, ErrorKind};

/// Drain for Loggers
///
/// Implementing this trait allows writing own Drains
pub trait Drain: Send + Sync {
    /// Write one logging record
    fn log(&self, info: &RecordInfo, &[OwnedKeyValue], &[BorrowedKeyValue]) -> Result<()>;
}

/// Drain discarding everything
pub struct Discard;

impl Drain for Discard {
    fn log(&self, _: &RecordInfo, _: &[OwnedKeyValue], _: &[BorrowedKeyValue]) -> Result<()> {
        Ok(())
    }
}

/// Drain formating records and writing them to a byte-stream (io::Write)
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer<W: io::Write, F: format::Format> {
    io: Mutex<W>,
    format: F,
}

impl<W: io::Write, F: format::Format> Streamer<W, F> {
    /// Create new `Streamer` writing to `io` using `format`
    pub fn new(io: W, format: F) -> Self {
        Streamer {
            io: Mutex::new(io),
            format: format,
        }
    }
}

impl<W: 'static + io::Write + Send, F: format::Format + Send> Drain for Streamer<W, F> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        let mut buf = Vec::with_capacity(128);
        try!(self.format.format(&mut buf, info, logger_values, values));
        {
            let mut io = try!(self.io.lock().map_err(|_| -> Error { ErrorKind::LockError.into() }));
            try!(io.write_all(&buf));
        }
        Ok(())
    }
}

/// Drain formating records and writing them to a byte-stream (io::Write)
/// asynchronously.
///
/// Internally, new thread will be spawned taking care of actually writing
/// the data.
pub struct AsyncStreamer<F: format::Format> {
    format: F,
    io: Mutex<AsyncIoWriter>,
}

impl<F: format::Format> AsyncStreamer<F> {
    /// Create new `AsyncStreamer` writing to `io` using `format`
    pub fn new<W: io::Write + Send + 'static>(io: W, format: F) -> Self {
        AsyncStreamer {
            io: Mutex::new(AsyncIoWriter::new(io)),
            format: format,
        }
    }
}

impl<F: format::Format + Send> Drain for AsyncStreamer<F> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        let mut buf = Vec::with_capacity(128);
        try!(self.format.format(&mut buf, info, logger_values, values));
        {
            let mut io = try!(self.io.lock().map_err(|_| -> Error { ErrorKind::LockError.into() }));
            try!(io.write_nocopy(buf));
        }
        Ok(())
    }
}

/// Filter log record
///
/// Wraps a drain and passes records to it, only if their `RecordInfo`
/// satisifies a condition `cond`.
pub struct Filter<D: Drain> {
    drain: D,
    // eliminated dynamic dispatch, after rust learns `-> impl Trait`
    cond: Box<Fn(&RecordInfo) -> bool + 'static + Send + Sync>,
}

impl<D: Drain> Filter<D> {
    /// Create Filter wrapping given `subdrain` and passing to it records
    /// only the `cond` is true
    pub fn new<F: 'static + Sync + Send + Fn(&RecordInfo) -> bool>(drain: D, cond: F) -> Self {
        Filter {
            drain: drain,
            cond: Box::new(cond),
        }
    }
}

impl<D: Drain> Drain for Filter<D> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        if (self.cond)(&info) {
            self.drain.log(info, logger_values, values)
        } else {
            Ok(())
        }
    }
}

/// Record log level filter
///
/// Wraps a drain and passes records to it, only
/// if their level is at least given level.
///
/// TODO: Remove this type. This drain is a special case of `Filter`, but
/// because `Filter` can not use static dispatch ATM due to Rust limitations
/// that will be lifted in the future, it is a standalone type.
pub struct FilterLevel<D: Drain> {
    level: Level,
    drain: D,
}

impl<D: Drain> FilterLevel<D> {
    /// Create `FilterLevel`
    pub fn new(drain: D, level: Level) -> Self {
        FilterLevel {
            level: level,
            drain: drain,
        }
    }
}

impl<D: Drain> Drain for FilterLevel<D> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        if info.level.is_at_least(self.level) {
            self.drain.log(info, logger_values, values)
        } else {
            Ok(())
        }
    }
}

/// Duplicate records into two drains
pub struct Duplicate<D1: Drain, D2: Drain> {
    drain1: D1,
    drain2: D2,
}


impl<D1: Drain, D2: Drain> Duplicate<D1, D2> {
    /// Create `Duplicate`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Duplicate {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1: Drain, D2: Drain> Drain for Duplicate<D1, D2> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        let res1 = self.drain1.log(info, logger_values, values);
        let res2 = self.drain2.log(info, logger_values, values);

        // TODO: Don't discard e2 in case of two errors at once?
        match (res1, res2) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e1), Err(_)) => Err(e1),
        }
    }
}

/// Failover drain
///
/// Log everything to logger `D1`, but in case of it reporting error,
/// try logging to `D2`. If it also returned an error, forward it up.
pub struct Failover<D1: Drain, D2: Drain> {
    drain1: D1,
    drain2: D2,
}

impl<D1: Drain, D2: Drain> Failover<D1, D2> {
    /// Create `Failover`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Failover {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1: Drain, D2: Drain> Drain for Failover<D1, D2> {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue])
           -> Result<()> {
        match self.drain1.log(info, logger_values, values) {
            Ok(_) => Ok(()),
            Err(_) => self.drain2.log(info, logger_values, values),
        }
    }
}


enum AsyncIoMsg {
    Bytes(Vec<u8>),
    Flush,
    Eof,
}

/// Asynchronous io::Writer
///
/// TODO: Publish as a different create / use existing one?
///
/// Wraps an `io::Writer` and writes to it in separate thread
/// using channel to send the data.
///
/// This makes logging not block on potentially-slow IO operations.
///
/// Note: Dropping `AsyncIoWriter` waits for it's io-thread to finish.
/// If you can't tolerate the delay, make sure to use `Logger::
struct AsyncIoWriter {
    sender: mpsc::Sender<AsyncIoMsg>,
    join: Option<thread::JoinHandle<()>>,
}

impl AsyncIoWriter {
    /// Create `AsyncIoWriter`
    pub fn new<W: io::Write + Send + 'static>(mut io: W) -> Self {
        let (tx, rx) = mpsc::channel();
        let join = thread::spawn(move || {
            loop {
                match rx.recv().unwrap() {
                    AsyncIoMsg::Bytes(buf) => io.write_all(&buf).unwrap(),
                    AsyncIoMsg::Flush => io.flush().unwrap(),
                    AsyncIoMsg::Eof => return,
                }
            }
        });

        AsyncIoWriter {
            sender: tx,
            join: Some(join),
        }
    }

    /// Write data to IO, without copying
    ///
    /// As an optimization, when `buf` is already an owned
    /// `Vec`, it can be sent over channel without copying.
    pub fn write_nocopy(&mut self, buf: Vec<u8>) -> Result<()> {
        try!(self.sender
            .send(AsyncIoMsg::Bytes(buf))
            .map_err(|_| -> Error { ErrorKind::SendError.into() }));
        Ok(())
    }
}

impl io::Write for AsyncIoWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let _ = self.sender.send(AsyncIoMsg::Bytes(buf.to_vec())).unwrap();
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        let _ = self.sender.send(AsyncIoMsg::Flush);
        Ok(())
    }
}


impl Drop for AsyncIoWriter {
    fn drop(&mut self) {
        let _ = self.sender.send(AsyncIoMsg::Eof);
        let _ = self.join.take().unwrap().join();
    }
}

/// Create `Streamer` drain
pub fn stream<W: io::Write + Send, F: format::Format>(io: W, format: F) -> Streamer<W, F> {
    Streamer::new(io, format)
}

/// Create `AsyncStreamer` drain
pub fn async_stream<W: io::Write + Send, F: format::Format>(io: W, format: F) -> Streamer<W, F> {
    Streamer::new(io, format)
}

/// Create a Discard drain
pub fn discard() -> Discard {
    Discard
}

/// Filter by `cond` closure
pub fn filter<D: Drain, F: 'static + Send + Sync + Fn(&RecordInfo) -> bool>(cond: F,
                                                                            d: D)
                                                                            -> Filter<D> {
    Filter::new(d, cond)
}

/// Filter by log level
pub fn filter_level<D: Drain>(level: Level, d: D) -> FilterLevel<D> {
    FilterLevel::new(d, level)
}

/// Create Duplicate drain
pub fn duplicate<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Duplicate<D1, D2> {
    Duplicate::new(d1, d2)
}

/// Create Failover drain
pub fn failover<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Failover<D1, D2> {
    Failover::new(d1, d2)
}
