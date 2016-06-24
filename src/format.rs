use super::logger::RecordInfo;
use super::ser::{Serialize, SerdeSerializer};
use super::Level;

use super::{BorrowedKeyValue,OwnedKeyValue};
use std::fmt::Write;

use serde_json;
use ansi_term::Colour;

/// Format record information
pub trait Format : Send+Sync+Sized {
    /// Format one logging record into `String`
    fn format(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], record_values : &[BorrowedKeyValue]) -> String;
}


/// ToJson formatter
///
/// Each record will be printed as a Json map.
pub struct Json {
    newlines : bool,
}


impl Json {
    /// Create new `Json` format.
    pub fn new() -> Self {
        Json {
            newlines : true,
        }
    }

    /// Create new `Json` format that does not add
    /// newlines after each record.
    pub fn new_nonewline() -> Self {
        Json {
            newlines : false,
        }
    }
}

impl Format for Json {
    fn format(&self, rinfo : &RecordInfo, logger_values : &[OwnedKeyValue], record_values : &[BorrowedKeyValue]) -> String {
        let mut serializer = serde_json::Serializer::new(vec!());
        {
            let mut serializer = &mut SerdeSerializer(&mut serializer);

            rinfo.level.as_str().serialize(rinfo, "level", serializer);
            rinfo.msg.serialize(rinfo, "msg", serializer);

            for &(ref k, ref v) in logger_values.iter() {
                v.serialize(rinfo, k, serializer);
            }

            for &(ref k, ref v) in record_values.iter() {
                v.serialize(rinfo, k, serializer);
            }
        }

        let ts = rinfo.ts.to_rfc3339();
        let formatted = if self.newlines {
            format!("{{\"ts\":{}{}}}\n", ts, String::from_utf8_lossy(&serializer.into_inner()))
        } else {
            format!("{{\"ts\":{}{}}}", ts, String::from_utf8_lossy(&serializer.into_inner()))
        };

        formatted
    }
}

/// Terminal formatting with optional color support
pub struct Terminal {
    color : bool,
}

impl Terminal {
    /// New Terminal format that prints using color
    pub fn colored() -> Self {
        Terminal {
            color: true,
        }
    }

    /// New Terminal format that prints without using color
    pub fn plain() -> Self {
        Terminal {
            color: false,
        }
    }
}

fn severity_to_color(lvl : Level) -> u8 {
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
    fn format(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], values : &[BorrowedKeyValue]) -> String {
        let color = Colour::Fixed(severity_to_color(info.level));

        let mut s = String::new();

        let _ = write!(s, "{:?}[{}] {}",
               info.ts,
               if self.color {
                   color.paint(info.level.as_str()).to_string()
               } else {
                   info.level.as_str().to_owned()
               },
               info.msg
               );


        for &(ref k, ref v) in logger_values {
            let _ = write!(s, ", ");
            v.serialize(info, k, &mut s);
        }

        for &(k,v) in values {
            let _ = write!(s, ", ");
            v.serialize(info, k, &mut s);
        }

        s.push_str("\n");

        s
    }
}
