//! Journald drain fro slog-rs
//!
//! Since Journald supports structured data, structured data passed to slog is
//! simply forwarded to Journald as structured data.
//!
//! # Examples
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_journald;
//!
//! use slog::*;
//! use slog_journald::*;
//!
//! fn main() {
//!     let root = Logger::root(JournaldDrain.fuse(), o!("build_di" => "12344"));
//!     info!(root, "Testing journald"; "foo" => "bar");
//! }
//! ```

#![warn(missing_docs)]

extern crate libsystemd_sys;
extern crate libc;
extern crate slog;

use std::ascii::AsciiExt;
use std::fmt::Display;
use std::os::raw::{c_int, c_void};

use libc::{LOG_CRIT, LOG_ERR, LOG_INFO, LOG_WARNING, LOG_NOTICE, LOG_DEBUG, size_t};
use libsystemd_sys::const_iovec;
use libsystemd_sys::journal::sd_journal_sendv;
use slog::{Drain, Record, OwnedKeyValueList, Level};
use slog::ser::Result as SerResult;

/// Drain records and send to journald as structured data.
///
/// Journald requires keys to be uppercase alphanumeric, so logging keys
/// are capitalized and all non-alpha-numeric letters are converted to underscores.
pub struct JournaldDrain;

impl Drain for JournaldDrain {
    type Error = ::Error;
    fn log(&self, info: &Record, logger_values: &OwnedKeyValueList) -> Result<(), ::Error> {
        let mut serializer = Serializer::new();
        serializer.add_field("PRIORITY", level_to_priority(info.level()));
        serializer.add_field("MESSAGE", info.msg());
        serializer.add_field("CODE_FILE", info.file());
        serializer.add_field("CODE_LINE", info.line());
        serializer.add_field("CODE_MODULE", info.module());
        serializer.add_field("CODE_FUNCTION", info.function());
        for (ref k, ref v) in logger_values.iter() {
            try!(v.serialize(info, k, &mut serializer));
        }
        for &(ref k, ref v) in info.values().iter() {
            try!(v.serialize(info, k, &mut serializer));
        }

        journald_send(serializer.fields.as_slice())
    }
}

/// Error type for logging to journald.
#[derive(Debug)]
pub enum Error {
    /// Error representing a non-zero return from `sd_journal_sendv`.
    ///
    /// The contained integer is the return value form `sd_journal_sendv`, which can
    /// be treated as an errno.
    Journald(i32),
    /// Error from serializing
    Serialization(slog::ser::Error),
}

impl Display for Error {
    fn fmt(&self, fmt: &mut std::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Journald(ref errno) => write!(fmt, "sd_journal_sendv returned {}", errno),
            Error::Serialization(ref e) => write!(fmt, "Unable to serialize item: {:?}", e),
        }
    }
}

impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Journald(_) => "Unable to send to journald",
            Error::Serialization(ref e) => e.description(),
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Journald(_) => None,
            Error::Serialization(ref e) => Some(e),
        }
    }
}

impl From<slog::ser::Error> for Error {
    fn from(e: slog::ser::Error) -> Error {
        Error::Serialization(e)
    }
}

fn journald_send(args: &[String]) -> Result<(), Error> {
    let iovecs = strings_to_iovecs(args);
    let ret = unsafe { sd_journal_sendv(iovecs.as_ptr(), iovecs.len() as c_int) };
    if ret == 0 {
        Ok(())
    } else {
        Err(Error::Journald(ret))
    }
}

fn level_to_priority(level: Level) -> c_int {
    match level {
        Level::Critical => LOG_CRIT,
        Level::Error => LOG_ERR,
        Level::Warning => LOG_WARNING,
        Level::Info => LOG_NOTICE,
        Level::Debug => LOG_INFO,
        Level::Trace => LOG_DEBUG,
    }
}

fn strings_to_iovecs(strings: &[String]) -> Vec<const_iovec> {
    strings.iter()
        .map(|s| {
            const_iovec {
                iov_base: s.as_ptr() as *const c_void,
                iov_len: s.len() as size_t,
            }
        })
        .collect()
}

/// Journald keys must consist only of uppercase letters, numbers
/// and underscores (but cannot begin with underscores).
// So we capitalize the string and remove any invalid characters
fn sanitize_key(key: &str) -> String {
    key.char_indices()
        .filter_map(|pair| {
            let (index, c) = pair;
            match c {
                'A'...'Z' | '0'...'9' => Some(c),
                'a'...'z' => Some(c.to_ascii_uppercase()),
                '_' if index != 1 => Some(c),
                _ => None,
            }
        })
        .collect()
}

struct Serializer {
    fields: Vec<String>,
}

impl Serializer {
    fn new() -> Serializer {
        Serializer { fields: Vec::new() }
    }
    fn add_field<T: Display>(&mut self, key: &str, val: T) {
        self.fields.push(format!("{}={}", key, val));
    }
    fn emit<T: Display>(&mut self, key: &str, val: T) -> SerResult {
        self.add_field(&sanitize_key(key), val);
        Ok(())
    }
}

impl slog::Serializer for Serializer {
    fn emit_bool(&mut self, key: &str, val: bool) -> SerResult {
        self.emit(key, val)
    }
    fn emit_unit(&mut self, key: &str) -> SerResult {
        self.emit(key, "")
    }
    fn emit_none(&mut self, key: &str) -> SerResult {
        self.emit(key, "None")
    }
    fn emit_char(&mut self, key: &str, val: char) -> SerResult {
        self.emit(key, val)
    }
    fn emit_u8(&mut self, key: &str, val: u8) -> SerResult {
        self.emit(key, val)
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> SerResult {
        self.emit(key, val)
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> SerResult {
        self.emit(key, val)
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> SerResult {
        self.emit(key, val)
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> SerResult {
        self.emit(key, val)
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> SerResult {
        self.emit(key, val)
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> SerResult {
        self.emit(key, val)
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> SerResult {
        self.emit(key, val)
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> SerResult {
        self.emit(key, val)
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> SerResult {
        self.emit(key, val)
    }
    fn emit_usize(&mut self, key: &str, val: usize) -> SerResult {
        self.emit(key, val)
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> SerResult {
        self.emit(key, val)
    }
    fn emit_str(&mut self, key: &str, val: &str) -> SerResult {
        self.emit(key, val)
    }
    fn emit_arguments(&mut self, key: &str, val: &std::fmt::Arguments) -> SerResult {
        self.emit(key, val)
    }
}
