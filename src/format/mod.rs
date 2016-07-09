use super::logger::RecordInfo;
use super::ser::SerdeSerializer;
use super::Level;

use super::{BorrowedKeyValue, OwnedKeyValue};
use std::fmt::Write;

use serde_json;
use ansi_term::Colour;

/// Format record information
pub trait Format: Send + Sync + Sized {
    /// Format one logging record into `String`
    fn format(&self,
              info: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue])
              -> String;
}


/// ToJson formatter
///
/// Each record will be printed as a Json map.
pub struct Json {
    newlines: bool,
    values: Vec<OwnedKeyValue>,
}

impl Json {
    /// Create new `Json` format.
    ///
    /// TODO: Add a builder pattern to configure newlines,
    /// and custom records.
    pub fn new() -> Self {
        Json {
            newlines: true,
            values: o!(
                "ts" => |rinfo : &RecordInfo| {
                    rinfo.ts.to_rfc3339()
                },
                "level" => |rinfo : &RecordInfo| {
                    rinfo.level.as_str()
                },
                "msg" => |rinfo : &RecordInfo| {
                    rinfo.msg.clone()
                }
                )
                        .to_vec(),
        }
    }

    /// Build a JSON formatter with custom settings
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

/// JSON formatter builder
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

    /// Build `JSON` format
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

/// Terminal formatting with optional color support
pub struct Terminal {
    color: bool,
}

impl Terminal {
    /// New Terminal format that prints using color
    pub fn colored() -> Self {
        Terminal { color: true }
    }

    /// New Terminal format that prints without using color
    pub fn plain() -> Self {
        Terminal { color: false }
    }
}

fn severity_to_color(lvl: Level) -> u8 {
    match lvl {
        Level::Critical => 35,
        Level::Error => 31,
        Level::Warning => 33,
        Level::Info => 32,
        Level::Debug => 36,
        Level::Trace => 0,
    }
}

impl Format for Terminal {
    fn format(&self,
              info: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              values: &[BorrowedKeyValue])
              -> String {
        let color = Colour::Fixed(severity_to_color(info.level));

        let mut s = String::new();

        let _ = write!(s,
                       "{:?}[{}] {}",
                       info.ts,
                       if self.color {
                           color.paint(info.level.as_str()).to_string()
                       } else {
                           info.level.as_str().to_owned()
                       },
                       info.msg);


        for &(ref k, ref v) in logger_values {
            let _ = write!(s, ", ");
            v.serialize(info, k, &mut s);
        }

        for &(k, v) in values {
            let _ = write!(s, ", ");
            v.serialize(info, k, &mut s);
        }

        s.push_str("\n");

        s
    }
}
