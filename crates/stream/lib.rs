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
use std::{mem, io, thread};
use slog::Record;

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
pub struct AsyncStreamer<F: Format> {
    format: F,
    io: AsyncIoWriter,
}

impl<F: Format> AsyncStreamer<F> {
    /// Create new `AsyncStreamer` writing to `io` using `format`
    pub fn new<W: io::Write + Send + 'static>(io: W, format: F) -> Self {
        AsyncStreamer {
            io: AsyncIoWriter::new(io),
            format: format,
        }
    }
}

impl<F: Format + Send> Drain for AsyncStreamer<F> {
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
                               let mut new_buf = Vec::with_capacity(128);
                               mem::swap(&mut *buf, &mut new_buf);
                               try!(self.io.write_nocopy(new_buf));
                           }
                           Ok(())

                       }}()
                   ;

                   if res.is_err() {
                       buf.clear();
                   }

                   res
               })
    }
}

enum AsyncIoMsg {
    Bytes(Vec<u8>),
    Eof,
}

/// Asynchronous `io::Write`r
///
/// TODO: Publish as a different crate / use existing one?
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
    pub fn new<W: io::Write + Send + 'static>(mut io: W) -> Self {
        let (tx, rx) = mpsc::channel();
        let join = thread::spawn(move || {
            loop {
                match rx.recv().unwrap() {
                    AsyncIoMsg::Bytes(buf) => io.write_all(&buf).unwrap(),
                    AsyncIoMsg::Eof => return,
                }
            }
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
    pub fn write_nocopy(&self, buf: Vec<u8>) -> io::Result<()> {
        let sender = self.get_sender();

        sender.send(AsyncIoMsg::Bytes(buf))
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))
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
pub fn async_stream<W: io::Write + Send + 'static, F: Format>(io: W, format: F) -> AsyncStreamer<F> {
    AsyncStreamer::new(io, format)
}

