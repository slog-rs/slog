//! Unix terminal formatter for slog-rs

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
use slog::format::Format;

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

/// Short for `Terminal::colored()`
pub fn format_colored() -> Terminal {
    Terminal::colored()
}

/// Short for `Terminal::plain()`
pub fn format_plain() -> Terminal {
    Terminal::plain()
}



/// Drain to `stdout`
///
/// Automatically using color if printing to tty
pub fn stdout() -> Streamer<io::Stdout, Terminal> {
    Streamer::new(
        io::stdout(),
        if stdout_isatty() {
            Terminal::colored()
        } else {
            Terminal::plain()
        }
    )
}

/// Drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn stderr() -> Streamer<io::Stderr, Terminal> {
    Streamer::new(
        io::stderr(),
        if stderr_isatty() {
            Terminal::colored()
        } else {
            Terminal::plain()
        }
    )
}

/// Asynchronous drain to `stdout`
///
/// Automatically using color if printing to tty
pub fn async_stdout() -> Streamer<AsyncIoWriter, Terminal> {
    Streamer::new(
        AsyncIoWriter::new(io::stdout()),
        if stdout_isatty() {
            Terminal::colored()
        } else {
            Terminal::plain()
        }
    )
}

/// Asynchronos drain to `stderr`
///
/// Automatically using color if output goes to tty
pub fn async_stderr() -> Streamer<AsyncIoWriter, Terminal> {
    Streamer::new(
        AsyncIoWriter::new(io::stderr()),
        if stderr_isatty() {
            Terminal::colored()
        } else {
            Terminal::plain()
        }
    )
}


