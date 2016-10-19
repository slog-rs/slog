//! `io::Write` streamer for slog-rs
//!
//! One of the main drains not included in the core `slog-rs` create.
//! `Streamer` drain serializes logging records into stream of bytes
//! using given `Format` and writes it to a given `io::Write`.
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate thread_local;

use slog::Drain;

use std::cell::RefCell;

use std::sync::{mpsc, Mutex};
use std::fmt;
use std::{io, thread};
use slog::{Record, RecordStatic, Level};
use slog::ser::{self, Serialize, Serializer};

use slog::OwnedKeyValueList;

include!("format.rs");

thread_local! {
    static TL_BUF: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(128))
}

/// Drain formating records and writing them to a byte-stream (`io::Write`)
///
/// Uses mutex to serialize writes to `io`. Use `AsyncStreamer` for better
/// performance, but without guarantee of immediate output.
pub struct Streamer<W: io::Write, F: Format> {
    io: Mutex<W>,
    format: F,
}

impl<W: io::Write, F: Format> Streamer<W, F> {
    /// Create new `Streamer` writing to `io` using `format`
    pub fn new(io: W, format: F) -> Self {
        Streamer {
            io: Mutex::new(io),
            format: format,
        }
    }
}

impl<W: 'static + io::Write + Send, F: Format + Send> Drain for Streamer<W, F> {
    type Error = io::Error;

    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
        -> io::Result<()> {

            TL_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                let res = {
                    || {
                        try!(self.format.format(&mut *buf, info, logger_values));
                        {
                            let mut io = try!(self.io.lock().map_err(|_| io::Error::new(io::ErrorKind::Other, "lock error")));
                            try!(io.write_all(&buf));
                        }
                        Ok(())
                    }
                }();
                buf.clear();
                res
            })
        }
}

/// Drain formating records and writing them to a byte-stream (`io::Write`)
/// asynchronously.
///
/// Internally, new thread will be spawned taking care of actually writing
/// the data.
pub struct AsyncStreamer {
    io: AsyncIoWriter,
}

impl AsyncStreamer {
    /// Create new `AsyncStreamer` writing to `io` using `format`
    pub fn new<W: io::Write + Send + 'static, F : Format+Send+'static>(io: W, format: F) -> Self {
        AsyncStreamer {
            io: AsyncIoWriter::new(io, format),
        }
    }
}

type RecordValues = Vec<(&'static str, Box<Serialize+Send>)>;

struct ToSendSerializer {
    record_values : RecordValues,
}

impl ToSendSerializer {
    fn new() -> Self {
        ToSendSerializer {
            record_values: Vec::new(),
        }
    }

    fn finish(self) -> RecordValues {
        self.record_values
    }
}

impl Serializer for ToSendSerializer {
    fn emit_bool(&mut self, key: &'static str, val: bool) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_unit(&mut self, key: &'static str) -> ser::Result
    { self.record_values.push((key, Box::new(()))); Ok(())}
    fn emit_none(&mut self, key: &'static str) -> ser::Result
    {
        let val : Option<()> = None;
        self.record_values.push((key, Box::new(val))); Ok(())
    }
    fn emit_char(&mut self, key: &'static str, val: char) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_u8(&mut self, key: &'static str, val: u8) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_i8(&mut self, key: &'static str, val: i8) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_u16(&mut self, key: &'static str, val: u16) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_i16(&mut self, key: &'static str, val: i16) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_u32(&mut self, key: &'static str, val: u32) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_i32(&mut self, key: &'static str, val: i32) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_f32(&mut self, key: &'static str, val: f32) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_u64(&mut self, key: &'static str, val: u64) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_i64(&mut self, key: &'static str, val: i64) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_f64(&mut self, key: &'static str, val: f64) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_usize(&mut self, key: &'static str, val: usize) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_isize(&mut self, key: &'static str, val: isize) -> ser::Result
    { self.record_values.push((key, Box::new(val))); Ok(())}
    fn emit_str(&mut self, key: &'static str, val: &str) -> ser::Result
    { self.record_values.push((key, Box::new(String::from(val)))); Ok(())}
    fn emit_arguments(&mut self, key: &'static str, val: &fmt::Arguments) -> ser::Result
    { self.record_values.push((key, Box::new(fmt::format(*val)))); Ok(())}
}


impl Drain for AsyncStreamer {
    type Error = io::Error;

    fn log(&self,
           record: &Record,
           logger_values: &OwnedKeyValueList)
           -> io::Result<()> {

               let mut ser = ToSendSerializer::new();
               for &(k, v) in record.values() {
                   try!(v.serialize(record, k, &mut ser))
               }

               self.io.send(
                   AsyncRecord {
                       msg: fmt::format(record.msg()),
                       level: record.level(),
                       file: record.file(),
                       line: record.line(),
                       column: record.column(),
                       function: record.function(),
                       module: record.module(),
                       target: String::from(record.target()),
                       logger_values: logger_values.clone(),
                       record_values: ser.finish(),
                   }
                   )
    }
}

struct AsyncRecord {
    msg: String,
    level: Level,
    file: &'static str,
    line: u32,
    column: u32,
    function: &'static str,
    module: &'static str,
    target: String,
    logger_values: OwnedKeyValueList,
    record_values: Vec<(&'static str, Box<Serialize+Send>)>,
}

enum AsyncIoMsg {
    Record(AsyncRecord),
    Eof,
}

/// Asynchronous `io::Write`r
///
/// Wraps an `io::Writer` and writes to it in separate thread
/// using channel to send the data.
///
/// This makes logging not block on potentially-slow IO operations.
///
/// Note: Dropping `AsyncIoWriter` waits for it's io-thread to finish. If you
/// can't tolerate the delay, make sure you `drop` it eg. in another thread.
struct AsyncIoWriter {
    ref_sender: Mutex<mpsc::Sender<AsyncIoMsg>>,
    tl_sender: thread_local::ThreadLocal<mpsc::Sender<AsyncIoMsg>>,
    join: Mutex<Option<thread::JoinHandle<()>>>,
}

impl AsyncIoWriter {
    /// Create `AsyncIoWriter`
    pub fn new<W: io::Write + Send + 'static, F : Format+Send+'static>(mut io: W, format : F) -> Self {
        let (tx, rx) = mpsc::channel();
        let join = thread::spawn(move || {
            TL_BUF.with(|buf| {
                let mut buf = buf.borrow_mut();
                loop {
                    match rx.recv().unwrap() {
                        AsyncIoMsg::Record(r) => {
                            let rs = RecordStatic {
                                level: r.level,
                                file: r.file,
                                line: r.line,
                                column: r.column,
                                function: r.function,
                                module: r.module,
                                target: &r.target,
                            };
                            let record_values : Vec<_> = r.record_values
                                .iter()
                                .map(|&(k, ref v)| (k, v as &Serialize)).
                                collect();
                            format.format(&mut *buf,
                                          &Record::new(
                                              &rs,
                                              format_args!("{}", r.msg),
                                              record_values.as_slice(),
                                              ),
                                              &r.logger_values,
                                              ).unwrap();
                            io.write_all(&mut *buf).unwrap();
                            buf.clear();
                        },
                        AsyncIoMsg::Eof => return,
                    }
                }
            })
        });

        AsyncIoWriter {
            ref_sender: Mutex::new(tx),
            tl_sender: thread_local::ThreadLocal::new(),
            join: Mutex::new(Some(join)),
        }
    }

    fn get_sender(&self) -> &mpsc::Sender<AsyncIoMsg> {
        self.tl_sender.get_or(|| {
            // TODO: Change to `get_or_try` https://github.com/Amanieu/thread_local-rs/issues/2
            Box::new(self.ref_sender.lock().unwrap().clone())
        })
    }
    /// Write data to IO, without copying
    ///
    /// As an optimization, when `buf` is already an owned
    /// `Vec`, it can be sent over channel without copying.
    pub fn send(&self, r : AsyncRecord) -> io::Result<()> {
        let sender = self.get_sender();

        sender.send(AsyncIoMsg::Record(r))
            .map_err(|_| io::Error::new(io::ErrorKind::BrokenPipe, "Send failed"))
    }
}

impl Drop for AsyncIoWriter {
    fn drop(&mut self) {
        let sender = self.get_sender();

        let _ = sender.send(AsyncIoMsg::Eof);
        let _ = self.join.lock().unwrap().take().unwrap().join();
    }
}

/// Stream logging records to IO
///
/// Create `Streamer` drain
pub fn stream<W: io::Write + Send, F: Format>(io: W, format: F) -> Streamer<W, F> {
    Streamer::new(io, format)
}

/// Stream logging records to IO asynchronously
///
/// Create `AsyncStreamer` drain
pub fn async_stream<W: io::Write + Send + 'static, F: Format + Send + 'static>(io: W, format: F) -> AsyncStreamer {
    AsyncStreamer::new(io, format)
}
