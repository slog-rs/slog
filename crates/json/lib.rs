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
//!             slog_json::default()
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
    /// Build a Json formatter
    pub fn new() -> FormatBuilder {
        FormatBuilder::new()
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
    pub fn set_newlines(mut self, enabled: bool) -> Self {
        self.newlines = enabled;
        self
    }

    /// Add custom values to be printed with this formatter
    pub fn add_key_values(mut self, mut values: Vec<OwnedKeyValue>) -> Self {
        self.values.extend(values.drain(..));
        self
    }

    /// Add custom values to be printed with this formatter
    pub fn add_key_value(mut self, value: OwnedKeyValue) -> Self {
        self.values.push(value);
        self
    }

    /// Add default key-values:
    /// * `ts` - timestamp
    /// * `level` - record logging level name
    /// * `msg` - msg - formatted logging message
    pub fn add_default_keys(self) -> Self {
        self.add_key_values(
            o!(
                "ts" => PushLazy(move |_ : &Record, ser : ValueSerializer| {
                    ser.serialize(chrono::Local::now().to_rfc3339())
                }),
                "level" => move |rinfo : &Record| {
                    level_to_string(rinfo.level())
                },
                "msg" => PushLazy(move |record : &Record, ser : ValueSerializer| {
                    ser.serialize(record.msg())
                })
              )
            )
    }
}

impl slog_stream::Format for Format {
    fn format(&self,
              io: &mut io::Write,
              rinfo: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {

        let io = {
            let serializer = serde_json::Serializer::new(io);
            let mut serializer = try!(SerdeSerializer::start(serializer));

            for &(ref k, ref v) in self.values.iter() {
                try!(v.serialize(rinfo, k, &mut serializer));
            }
            for &(ref k, ref v) in logger_values.iter() {
                try!(v.serialize(rinfo, k, &mut serializer));
            }

            for &(ref k, ref v) in rinfo.values().iter() {
                try!(v.serialize(rinfo, k, &mut serializer));
            }
            let (serializer, res) = serializer.end();

            let _ = try!(res);
            serializer.into_inner()
        };
        if self.newlines {
            let _ = try!(io.write_all("\n".as_bytes()));
        }
        Ok(())
    }
}

/// Create new `FormatBuilder` to create `Format`
pub fn new() -> FormatBuilder {
    Format::new()
}

/// Default json `Format`
pub fn default() -> Format {
    Format::new().add_default_keys().build()
}
