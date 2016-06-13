use super::logger::RecordInfo;
use super::ser::{Serialize, SerdeSerializer};
use super::Level;

use super::{BorrowedKeyValue,OwnedKeyValue};
use std::fmt::Write;

use serde;
use serde_json;
use ansi_term::Colour;

use serde::Serializer as SerdeSerializerTrait;

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
    fn format(&self, info : &RecordInfo, logger_values : &[OwnedKeyValue], record_values : &[BorrowedKeyValue]) -> String {
        let mut serializer = serde_json::Serializer::new(vec!());

        let visitor = RecordVisitor {
            info : info,
            logger_values : logger_values,
            record_values : record_values,
            index : 0,
        };

        let _ = serializer.serialize_map(visitor);
        let mut formatted = serializer.into_inner();
        if self.newlines {
            formatted.push('\n' as u8);
        }

        String::from_utf8_lossy(&formatted).into_owned()
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
            v.serialize(k, &mut s);
        }

        for &(k,v) in values {
            let _ = write!(s, ", ");
            v.serialize(k, &mut s);
        }

        s.push_str("\n");

        s
    }
}

struct RecordVisitor<'a> {
    info : &'a RecordInfo,
    logger_values : &'a[OwnedKeyValue],
    record_values : &'a[BorrowedKeyValue<'a>],
    index : usize,
}

impl<'a> serde::ser::MapVisitor for RecordVisitor<'a> {
    fn visit<S>(&mut self, serializer: &mut S) -> Result<Option<()>, S::Error> where S: serde::Serializer {
        let ret = match self.index {
            0 => {self.info.level.as_str().serialize("level", &mut SerdeSerializer(serializer)); Ok(Some(()))},
            1 => {format!("{:?}", self.info.ts).as_str().serialize("ts", &mut SerdeSerializer(serializer)); Ok(Some(()))},
            2 => {self.info.msg.serialize("msg", &mut SerdeSerializer(serializer)); Ok(Some(()))},
            _ => if self.logger_values.len() + 3 < self.index{
                let (ref key, ref val) = self.logger_values[self.index - 3];
                val.serialize(&key, &mut SerdeSerializer(serializer));
                Ok(Some(()))
            } else if self.record_values.len() + 3 + self.logger_values.len() < self.index  {
                let (key, val) = self.record_values[self.index - 3 - self.logger_values.len()];
                val.serialize(key, &mut SerdeSerializer(serializer));
                Ok(Some(()))
            } else {
                Ok(None)
            }
        };
        self.index += 1;
        ret
    }

    fn len(&self) -> Option<usize> {
        Some(self.logger_values.len() + self.record_values.len() + 3)
    }
}
