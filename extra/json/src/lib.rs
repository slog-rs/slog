//! [Bunyan](https://www.npmjs.com/package/bunyan) formatting for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_json;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!     root.set_drain(
//!             drain::stream(
//!                 std::io::stderr(),
//!                 slog_json::new()
//!                 )
//!             );
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_serde;
extern crate serde_json;

use slog_serde::SerdeSerializer;

use slog::logger::RecordInfo;
use slog::{Level, OwnedKeyValue, BorrowedKeyValue};
use slog::format::Format;
use slog::Level::*;

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
pub struct Json {
    newlines: bool,
    values: Vec<OwnedKeyValue>,
}

impl Json {
    /// Create new `Json` format.
    ///
    /// It comes with some default fields (`ts`, `level`, `msg`)
    /// and uses newlines.
    ///
    /// Use `Json::build()` to build a custom Json formatter from
    /// scratch.
    pub fn new() -> Self {
        Json {
            newlines: true,
            values: o!(
                "ts" => |rinfo : &RecordInfo| {
                    rinfo.ts.to_rfc3339()
                },
                "level" => |rinfo : &RecordInfo| {
                    level_to_string(rinfo.level)
                },
                "msg" => |rinfo : &RecordInfo| {
                    rinfo.msg.clone()
                }
                ).to_vec(),
        }
    }

    /// Build a Json formatter with custom settings
    pub fn build() -> JsonBuilder {
        JsonBuilder::new()
    }

    /// Create new `Json` format that does not add
    /// newlines after each record.
    pub fn new_nonewline() -> Self {
        let mut json = Json::new();
        json.newlines = false;
        json
    }
}

/// Json formatter builder
///
/// Create with `Json::build`.
pub struct JsonBuilder {
    newlines: bool,
    values: Vec<OwnedKeyValue>,
}

impl JsonBuilder {
    fn new() -> Self {
        JsonBuilder {
            newlines: true,
            values: vec![],
        }
    }

    /// Build `Json` format
    ///
    /// This consumes the builder.
    pub fn build(self) -> Json {
        Json {
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
    pub fn add_key_values(&mut self, values: &[OwnedKeyValue]) -> &mut Self {
        self.values.extend_from_slice(values);
        self
    }

    /// Add custom values to be printed with this formatter
    pub fn add_key_value(&mut self, value: OwnedKeyValue) -> &mut Self {
        self.values.push(value);
        self
    }
}

impl Format for Json {
    fn format(&self,
              rinfo: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue])
              -> String {
        let mut serializer = serde_json::Serializer::new(vec![]);
        {
            let mut serializer = &mut SerdeSerializer(&mut serializer);
            for &(ref k, ref v) in self.values.iter() {
                v.serialize(rinfo, k, serializer);
            }
            for &(ref k, ref v) in logger_values.iter() {
                v.serialize(rinfo, k, serializer);
            }

            for &(ref k, ref v) in record_values.iter() {
                v.serialize(rinfo, k, serializer);
            }
        }

        // TODO: Optimize this part
        let mut inner_bytes = serializer.into_inner();
        {
            let empty = inner_bytes.is_empty();
            if empty {
                inner_bytes.push(',' as u8);
            }
        }
        let inner_str = String::from_utf8_lossy(&inner_bytes);

        if self.newlines {
            format!("{{{}}}\n", &inner_str[1..])
        } else {
            format!("{{{}}}", &inner_str[1..])
        }
    }
}

/// Short for `Json::new()`
pub fn new() -> Json {
    Json::new()
}

/// Short for `Json::build()`
pub fn build() -> JsonBuilder {
    Json::build()
}
