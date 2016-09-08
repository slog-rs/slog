use std::sync::{Mutex, mpsc};
use std::{mem, io, thread};

/// Logging drain
///
/// Drains generally mean destination for logs, but slog generalize the
/// term. `Drain`-s are responsible for filtering, formatting and writing the
/// log records into given destination.
///
/// Implementing this trait allows writing own Drains, that can be combined
/// with other drains.
pub trait Drain: Send + Sync {
    /// Write one logging record
    /// As an optimization (avoiding allocations), loggers are responsible for
    /// providing a byte buffer, that `Drain` can use for their own needs.
    fn log(&self, buf: &mut Vec<u8>, info: &Record, &OwnedKeyValueList) -> io::Result<()>;
}

impl<D: Drain+?Sized> Drain for Box<D> {
    fn log(&self, buf: &mut Vec<u8>, info: &Record, o: &OwnedKeyValueList) -> io::Result<()> {
        (**self).log(buf, info, o)
    }
}

impl<D: Drain+?Sized> Drain for Arc<D> {
    fn log(&self, buf: &mut Vec<u8>, info: &Record, o: &OwnedKeyValueList) -> io::Result<()> {
        (**self).log(buf, info, o)
    }
}

/// Drain discarding everything
pub struct Discard;

impl Drain for Discard {
    fn log(&self, _: &mut Vec<u8>, _: &Record, _: &OwnedKeyValueList) -> io::Result<()> {
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
           mut buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {

        let res =
            {
                || {
                    try!(self.format.format(&mut buf, info, logger_values));
                    {
                        let mut io = try!(self.io
                            .lock()
                            .map_err(|_| io::Error::new(io::ErrorKind::Other, "lock error")));
                        try!(io.write_all(&buf));
                    }
                    Ok(())
                }
            }();
        buf.clear();
        res
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
           mut buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        try!(self.format.format(&mut buf, info, logger_values));
        {
            let mut io = try!(self.io.lock().map_err(|_| io::Error::new(io::ErrorKind::Other, "lock error")));
            let mut new_buf = Vec::with_capacity(128);
            mem::swap(buf, &mut new_buf);
            try!(io.write_nocopy(new_buf));
        }
        Ok(())
    }
}

/// Drain filtering records
///
/// Wraps a `Drain` and passes `Record`-s to it, only if they satisifies a
/// condition `cond`.
pub struct Filter<D: Drain> {
    drain: D,
    // eliminated dynamic dispatch, after rust learns `-> impl Trait`
    cond: Box<Fn(&Record) -> bool + 'static + Send + Sync>,
}

impl<D: Drain> Filter<D> {
    /// Create Filter wrapping given `subdrain` and passing to it records
    /// only the `cond` is true
    pub fn new<F: 'static + Sync + Send + Fn(&Record) -> bool>(drain: D, cond: F) -> Self {
        Filter {
            drain: drain,
            cond: Box::new(cond),
        }
    }
}

impl<D: Drain> Drain for Filter<D> {
    fn log(&self,
           buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        if (self.cond)(&info) {
            self.drain.log(buf, info, logger_values)
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
pub struct LevelFilter<D: Drain> {
    level: Level,
    drain: D,
}

impl<D: Drain> LevelFilter<D> {
    /// Create `LevelFilter`
    pub fn new(drain: D, level: Level) -> Self {
        LevelFilter {
            level: level,
            drain: drain,
        }
    }
}

impl<D: Drain> Drain for LevelFilter<D> {
    fn log(&self,
           buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        if info.level().is_at_least(self.level) {
            self.drain.log(buf, info, logger_values)
        } else {
            Ok(())
        }
    }
}

/// Drain duplicating records to two sub-drains
///
/// Can be nested for more than two outputs.
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
           buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        let res1 = self.drain1.log(buf, info, logger_values);
        buf.clear();
        let res2 = self.drain2.log(buf, info, logger_values);

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
           buf: &mut Vec<u8>,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {
        match self.drain1.log(buf, info, logger_values) {
            Ok(_) => Ok(()),
            Err(_) => self.drain2.log(buf, info, logger_values),
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
/// TODO: Publish as a different crate / use existing one?
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
    pub fn write_nocopy(&mut self, buf: Vec<u8>) -> io::Result<()> {
        try!(self.sender
            .send(AsyncIoMsg::Bytes(buf))
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e)));
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

/// Stream logging records to IO
///
/// Create `Streamer` drain
pub fn stream<W: io::Write + Send, F: format::Format>(io: W, format: F) -> Streamer<W, F> {
    Streamer::new(io, format)
}

/// Stream logging records to IO asynchronously
///
/// Create `AsyncStreamer` drain
pub fn async_stream<W: io::Write + Send + 'static, F: format::Format>(io: W, format: F) -> AsyncStreamer<F> {
    AsyncStreamer::new(io, format)
}

/// Discard all logging records
///
/// Create a Discard drain
pub fn discard() -> Discard {
    Discard
}

/// Filter by `cond` closure
pub fn filter<D: Drain, F: 'static + Send + Sync + Fn(&Record) -> bool>(cond: F,
                                                                            d: D)
                                                                            -> Filter<D> {
    Filter::new(d, cond)
}

/// Filter by log level
pub fn level_filter<D: Drain>(level: Level, d: D) -> LevelFilter<D> {
    LevelFilter::new(d, level)
}

/// Duplicate records to two drains
///
/// Create `Duplicate` drain.
///
/// Can be nested for multiple outputs.
pub fn duplicate<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Duplicate<D1, D2> {
    Duplicate::new(d1, d2)
}

/// Failover logging to secondary drain on primary's failure
///
/// Create `Failover` drain
pub fn failover<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Failover<D1, D2> {
    Failover::new(d1, d2)
}
