//! Unix terminal formatter and drain for slog-rs
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//!
//! use slog::*;
//! use slog::IntoLogger;
//!
//! fn main() {
//!     let root = slog_term::async_stderr().into_logger(o!("build-id" => "8dfljdf"));
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate isatty;
extern crate ansi_term;
extern crate rustc_serialize;

use std::io;
use rustc_serialize::hex::ToHex;

use ansi_term::Colour;
use isatty::{stderr_isatty, stdout_isatty};

use slog::Record;
use slog::{Streamer, AsyncStreamer};
use slog::{Level, OwnedKeyValueNode};
use slog::format::Format as SlogFormat;

/// Format formatting with optional color support
pub struct Format {
    color: bool,
}

impl Format {
    /// New Format format that prints using color
    pub fn colored() -> Self {
        Format { color: true }
    }

    /// New Format format that prints without using color
    pub fn plain() -> Self {
        Format { color: false }
    }
}

fn severity_to_color(lvl: Level) -> u8 {
    match lvl {
        Level::Critical => 1,
        Level::Error => 9,
        Level::Warning => 3,
        Level::Info => 2,
        Level::Debug => 6,
        Level::Trace => 4,
    }
}

struct Serializer<W> {
    io: W,
    key: ansi_term::Style,
}

impl<W: io::Write> Serializer<W> {
    fn new(io: W, color: bool) -> Self {
        Serializer {
            io: io,
            key: if color {
                ansi_term::Style::new().bold()
            } else {
                ansi_term::Style::new()
            },
        }
    }

    fn maybe_comma(&mut self) -> std::io::Result<()> {
        // since we don't use it for first element, add comma
        // unconditionally
        try!(write!(self.io, ", "));
        Ok(())
    }

    fn into_inner(self) -> W {
        self.io
    }
}

impl<W: io::Write> slog::ser::Serializer for Serializer<W> {
    fn emit_none(&mut self, key: &str) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), "None"));
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: ()", self.key.paint(key)));
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_bytes(&mut self, key: &str, val: &[u8]) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val.to_hex()));
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> slog::ser::Result<()> {
        try!(self.maybe_comma());
        try!(write!(self.io, "{}: {}", self.key.paint(key), val));
        Ok(())
    }
}

impl SlogFormat for Format {
    fn format(&self,
              io: &mut io::Write,
              info: &Record,
              logger_values: &OwnedKeyValueNode)
              -> slog::format::Result<()> {
        let level_color = Colour::Fixed(severity_to_color(info.level()));
        let bold = ansi_term::Style::new().bold();

        if self.color {
            let _ = try!(write!(io,
                                "{} {} {}",
                                info.ts().format("%b %d %H:%M:%S%.3f"),
                                level_color.paint(info.level().as_short_str()),
                                bold.paint(info.msg()).to_string()));
        } else {
            let _ = try!(write!(io,
                                "{} {} {}",
                                info.ts().format("%b %d %H:%M:%S%.3f"),
                                info.level().as_short_str(),
                                info.msg()));
        }

        let mut serializer = Serializer::new(io, self.color);

        for &(ref k, ref v) in logger_values.iter() {
            try!(v.serialize(info, k, &mut serializer));
        }

        for &(k, v) in info.values() {
            try!(v.serialize(info, k, &mut serializer));
        }
        let mut io = serializer.into_inner();

        let _ = try!(write!(io, "\n"));

        Ok(())
    }
}

/// Short for `Format::colored()`
pub fn format_colored() -> Format {
    Format::colored()
}

/// Short for `Format::plain()`
pub fn format_plain() -> Format {
    Format::plain()
}



/// Drain to `stdout`
///
/// Automatically using color if printing to tty
pub fn stdout() -> Streamer<io::Stdout, Format> {
    Streamer::new(io::stdout(),
                  if stdout_isatty() {
                      Format::colored()
                  } else {
                      Format::plain()
                  })
}

/// Drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn stderr() -> Streamer<io::Stderr, Format> {
    Streamer::new(io::stderr(),
                  if stderr_isatty() {
                      Format::colored()
                  } else {
                      Format::plain()
                  })
}

/// Asynchronous drain to `stdout`
///
/// Automatically using color if printing to tty
pub fn async_stdout() -> AsyncStreamer<Format> {
    AsyncStreamer::new(io::stdout(),
                       if stdout_isatty() {
                           Format::colored()
                       } else {
                           Format::plain()
                       })
}

/// Asynchronos drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn async_stderr() -> AsyncStreamer<Format> {
    AsyncStreamer::new(io::stderr(),
                       if stderr_isatty() {
                           Format::colored()
                       } else {
                           Format::plain()
                       })
}
