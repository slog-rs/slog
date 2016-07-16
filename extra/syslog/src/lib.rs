//! Syslog drain for slog-rs
//!
//! WARNING: This crate needs some improvements.
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_syslog;
//!
//! use slog::*;
//! use slog_syslog::Facility;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!     root.set_drain(
//!             slog_syslog::unix_3164(
//!                 Facility::LOG_USER,
//!                 )
//!             );
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate syslog;
extern crate nix;
extern crate rustc_serialize as serialize;

use slog::format;
use slog::format::Format;
use slog::drain;
use slog::Level;
use slog::ser::Serializer;
use slog::logger::RecordInfo;
use slog::{OwnedKeyValue, BorrowedKeyValue};
use std::io;
use std::sync::Mutex;
use serialize::hex::ToHex;

pub use syslog::Facility;

fn level_to_severity(level : slog::Level) -> syslog::Severity {
    match level {
        Level::Critical => syslog::Severity::LOG_CRIT,
        Level::Error => syslog::Severity::LOG_ERR,
        Level::Warning => syslog::Severity::LOG_WARNING,
        Level::Info => syslog::Severity::LOG_NOTICE,
        Level::Debug => syslog::Severity::LOG_INFO,
        Level::Trace => syslog::Severity::LOG_DEBUG,
    }

}

/// Drain formatting records and writing them to a syslog ``Logger`
///
/// Uses mutex to serialize writes.
/// TODO: Add one that does not serialize?
pub struct Streamer3164 {
    io: Mutex<Box<syslog::Logger>>,
    format : Format3164,
}

impl Streamer3164 {
    /// Create new syslog ``Streamer` using given `format`
    pub fn new(logger : Box<syslog::Logger>) -> Self {
        Streamer3164 {
            io: Mutex::new(logger),
            format : Format3164::new(),
        }
    }
}

impl drain::Drain for Streamer3164 {
    fn log(&self,
           info: &RecordInfo,
           logger_values: &[OwnedKeyValue],
           values: &[BorrowedKeyValue]) {
        let mut buf = Vec::with_capacity(128);
        self.format.format(&mut buf, info, logger_values, values);
        let sever = level_to_severity(info.level);
        {
            let io = self.io.lock().unwrap();
            io.send(sever, &String::from_utf8_lossy(&buf)).unwrap();
        }
    }
}

/// Formatter to format defined in RFC 3164
pub struct Format3164;

impl Format3164 {
    /// Create new `Format3164`
    pub fn new() -> Self {
        Format3164
    }
}

impl format::Format for Format3164 {
    fn format(&self,
              io : &mut io::Write,
              rinfo: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue]) {
        let mut ser = KSV::new(io, "=".into());
        {
            for &(ref k, ref v) in logger_values.iter() {
                v.serialize(rinfo, k, &mut ser);
                let _ = ser.io().write_all(" ".as_bytes());
            }

            for &(ref k, ref v) in record_values.iter() {
                v.serialize(rinfo, k, &mut ser);
                let _ = ser.io().write_all(" ".as_bytes());
            }
        }
    }
}

/// Key-Separator-Value serializer
struct KSV<W : io::Write> {
    separator : String,
    io : W,
}

impl<W : io::Write> KSV<W> {
    fn new(io : W, separator : String) -> Self {
        KSV {
            io: io,
            separator : separator,
        }
    }

    fn io(&mut self) -> &mut W {
        &mut self.io
    }
}

impl<W : io::Write> Serializer for KSV<W> {
    fn emit_none(&mut self, key: &str) {
        write!(self.io, "{}{}{}", key, self.separator, "None").unwrap()
    }
    fn emit_unit(&mut self, key: &str) {
        write!(self.io, "{}", key).unwrap()
    }

    fn emit_bool(&mut self, key: &str, val: bool) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }

    fn emit_char(&mut self, key: &str, val: char) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_bytes(&mut self, key: &str, val: &[u8]) {
        write!(self.io, "{}{}{}", key, self.separator, val.to_hex()).unwrap()
    }

    fn emit_usize(&mut self, key: &str, val: usize) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_isize(&mut self, key: &str, val: isize) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }

    fn emit_u8(&mut self, key: &str, val: u8) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_i8(&mut self, key: &str, val: i8) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_u16(&mut self, key: &str, val: u16) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_i16(&mut self, key: &str, val: i16) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_u32(&mut self, key: &str, val: u32) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_i32(&mut self, key: &str, val: i32) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_f32(&mut self, key: &str, val: f32) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_u64(&mut self, key: &str, val: u64) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_i64(&mut self, key: &str, val: i64) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_f64(&mut self, key: &str, val: f64) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
    fn emit_str(&mut self, key: &str, val: &str) {
        write!(self.io, "{}{}{}", key, self.separator, val).unwrap()
    }
}

/// ``Streamer` to Unix syslog using RFC 3164 format
pub fn unix_3164(facility : syslog::Facility) -> Streamer3164 {
    Streamer3164::new(
        syslog::unix(facility).unwrap(),
        )
}
