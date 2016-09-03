use super::Record;

use super::OwnedKeyValueList;

use std::{io, fmt};

#[allow(missing_docs)]
mod error {
    use std::io;
    use super::super::ser;

    error_chain! {
        types {
            Error, ErrorKind, ChainErr, Result;
        }
        links {
            ser::Error, ser::ErrorKind, Serialization;
        }
        foreign_links {
            io::Error, Io, "io error";
        }
        errors {}
    }
}

pub use self::error::{Error, Result, ErrorKind};

/// Formats `Record`-s into IO
pub trait Format: Send + Sync + Sized {
    /// Format one logging record and write into `io`
    fn format(&self,
              io: &mut io::Write,
              info: &Record,
              logger_values: &OwnedKeyValueList)
              -> Result<()>;
}

/// Formatted stream decorator
///
/// Some `Format`s for which it make sense can use this to parametrize
/// themselves on the exact behavior of formatting parts of the output. This
/// can be used eg. to use color when displaying logger records on the terminal.
pub trait Decorator : Send + Sync + Sized {
    /// Per-record decorator
    type RecordDecorator : RecordDecorator;

    /// Get a `RecordDecorator` for a given `record`
    fn decorate(&self, record : &Record) -> Self::RecordDecorator;
}

/// Per-record decorator
pub trait RecordDecorator {
    /// Format a field
    fn fmt_msg(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
    /// Format a key
    fn fmt_key(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
    /// Format a separator
    fn fmt_separator(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
    /// Format a value
    fn fmt_value(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
    /// Format a timestamp
    fn fmt_timestamp(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
    /// Format a level
    fn fmt_level(&self, io : &mut io::Write, args : fmt::Arguments) -> io::Result<()> {
        io.write_fmt(args)
    }
}
