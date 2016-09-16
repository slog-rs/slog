//! Json formatter for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_json;
//! extern crate slog_stream;
//!
//! use slog::DrainExt;
//!
//! fn main() {
//!     let root = slog::Logger::root(
//!         slog_stream::stream(
//!             std::io::stderr(),
//!             slog_json::new()
//!         ).fuse(),
//!         o!("build-id" => "8dfljdf")
//!     );
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_serde;
extern crate slog_stream;
extern crate serde_json;
extern crate chrono;

use std::io;

use slog_serde::SerdeSerializer;
use slog::Record;
use slog::{Level, OwnedKeyValue, OwnedKeyValueList};
use slog::Level::*;
use slog::ser::{PushLazy, ValueSerializer};

fn level_to_string(level: Level) -> &'static str {
    match level {
        Critical => "CRIT",
        Error => "ERRO",
        Warning => "WARN",
        Info => "INFO",
        Debug => "DEBG",
        Trace => "TRCE",
    }
}

/// Json formatter
///
/// Each record will be printed as a Json map.
pub struct Format {
    newlines: bool,
    values: Vec<OwnedKeyValue>,
}

impl Format {
    /// Create new Json formatter
    ///
    /// It comes with some default fields (`ts`, `level`, `msg`)
    /// and uses newlines.
    ///
    /// Use `Format::build()` to build a custom Json formatter from
    /// scratch.
    pub fn new() -> Self {
        Format {
            newlines: true,
            values: o!(
                "ts" => PushLazy(move |_ : &Record, ser : ValueSerializer| {
                   ser.serialize(chrono::Local::now().to_rfc3339())
                }),
                "level" => move |rinfo : &Record| {
                    level_to_string(rinfo.level())
                },
                "msg" => PushLazy(move |record : &Record, ser : ValueSerializer| {
                   ser.serialize(record.msg())
                })
                ),
        }
    }

    /// Build a Json formatter with custom settings
    pub fn build() -> FormatBuilder {
        FormatBuilder::new()
    }

    /// Create new `Json` format that does not add
    /// newlines after each record.
    pub fn new_nonewline() -> Self {
        let mut json = Format::new();
        json.newlines = false;
        json
    }
}

/// Json formatter builder
///
/// Create with `Format::build`.
pub struct FormatBuilder {
    newlines: bool,
    values: Vec<OwnedKeyValue>,
}

impl FormatBuilder {
    fn new() -> Self {
        FormatBuilder {
            newlines: true,
            values: vec![],
        }
    }

    /// Build `Json` format
    ///
    /// This consumes the builder.
    pub fn build(self) -> Format {
        Format {
            values: self.values,
            newlines: self.newlines,
        }
    }

    /// Set writing a newline after ever log record
    pub fn set_newlines(&mut self, enabled: bool) -> &mut Self {
        self.newlines = enabled;
        self
    }

    /// Add custom values to be printed with this formatter
    pub fn add_key_values(&mut self, mut values: Vec<OwnedKeyValue>) -> &mut Self {
        self.values.extend(values.drain(..));
        self
    }

    /// Add custom values to be printed with this formatter
    pub fn add_key_value(&mut self, value: OwnedKeyValue) -> &mut Self {
        self.values.push(value);
        self
    }
}

// TODO: Get rid of after
// https://github.com/serde-rs/serde/issues/386
// is implemented
struct SkipFirstByte<W> {
    first: bool,
    io: W,
}

impl<W: io::Write> SkipFirstByte<W> {
    fn new(io: W) -> Self {
        SkipFirstByte {
            first: true,
            io: io,
        }
    }

    fn into_inner(self) -> W {
        self.io
    }
}

impl<W: io::Write> io::Write for SkipFirstByte<W> {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        if self.first && !buf.is_empty() {
            self.first = false;
            try!(self.io.write_all(&buf[1..]));
        } else {
            try!(self.io.write_all(buf))
        }
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        self.io.flush()
    }
}

impl slog_stream::Format for Format {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        let _ = try!(write!(io, "{{"));
        let mut serializer = serde_json::Serializer::new(SkipFirstByte::new(io));
        {
            let mut serializer = &mut SerdeSerializer(&mut serializer);

            for &(ref k, ref v) in self.values.iter() {
                try!(v.serialize(rinfo, k, serializer));
            }
            for &(ref k, ref v) in logger_values.iter() {
                try!(v.serialize(rinfo, k, serializer));
            }

            for &(ref k, ref v) in rinfo.values().iter() {
                try!(v.serialize(rinfo, k, serializer));
            }
        }
        let mut io = serializer.into_inner().into_inner();
        let _ = try!(io.write_all("}".as_bytes()));
        if self.newlines {
            let _ = try!(io.write_all("\n".as_bytes()));
        }
        Ok(())
    }
}

/// Build a default Json formatter
///
/// See `Format::new()` for details
pub fn new() -> Format {
    Format::new()
}

/// Use builder to create a custom Json formatter
///
/// See `FormatBuilder` for details
pub fn build() -> FormatBuilder {
    Format::build()
}
