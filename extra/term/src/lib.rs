//! Unix terminal formatter for slog-rs
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!     root.set_drain(slog_term::async_stderr());
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate isatty;
extern crate ansi_term;

use std::io;
use std::fmt::Write;

use ansi_term::Colour;
use isatty::{stderr_isatty, stdout_isatty};

use slog::logger::RecordInfo;
use slog::drain::{Streamer, AsyncIoWriter};
use slog::{Level, OwnedKeyValue, BorrowedKeyValue};
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
        Level::Critical => 35,
        Level::Error => 31,
        Level::Warning => 33,
        Level::Info => 32,
        Level::Debug => 36,
        Level::Trace => 0,
    }
}

impl SlogFormat for Format {
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
                           color.paint(info.level.as_short_str()).to_string()
                       } else {
                           info.level.as_short_str().to_owned()
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
    Streamer::new(
        io::stdout(),
        if stdout_isatty() {
            Format::colored()
        } else {
            Format::plain()
        }
    )
}

/// Drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn stderr() -> Streamer<io::Stderr, Format> {
    Streamer::new(
        io::stderr(),
        if stderr_isatty() {
            Format::colored()
        } else {
            Format::plain()
        }
    )
}

/// Asynchronous drain to `stdout`
///
/// Automatically using color if printing to tty
pub fn async_stdout() -> Streamer<AsyncIoWriter, Format> {
    Streamer::new(
        AsyncIoWriter::new(io::stdout()),
        if stdout_isatty() {
            Format::colored()
        } else {
            Format::plain()
        }
    )
}

/// Asynchronos drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn async_stderr() -> Streamer<AsyncIoWriter, Format> {
    Streamer::new(
        AsyncIoWriter::new(io::stderr()),
        if stderr_isatty() {
            Format::colored()
        } else {
            Format::plain()
        }
    )
}


