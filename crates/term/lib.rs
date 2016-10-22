//! Unix terminal formatter and drain for slog-rs
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_term;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::root(slog_term::streamer().build().fuse(), o!("build-id" => "8dfljdf"));
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_stream;
extern crate isatty;
extern crate chrono;

use std::{io, fmt, sync};

use isatty::{stderr_isatty, stdout_isatty};

use slog::Record;
use slog::ser;
use slog::{Level, OwnedKeyValueList};
use slog_stream::Format as StreamFormat;
use slog_stream::{Decorator, RecordDecorator, stream, async_stream};

/// Timestamp function type
pub type TimestampFn = Fn(&mut io::Write) -> io::Result<()> + Send + Sync;

/// Formatting mode
pub enum FormatMode {
    /// Compact logging format
    Compact,
    /// Full logging format
    Full,
}

/// Full formatting with optional color support
pub struct Format<D: Decorator> {
    mode: FormatMode,
    decorator: D,
    history: sync::Mutex<Vec<usize>>,
    fn_timestamp: Box<TimestampFn>,
}

impl<D: Decorator> Format<D> {
    /// New Format format that prints using color
    pub fn new(mode: FormatMode, d: D, fn_timestamp: Box<TimestampFn>) -> Self {
        Format {
            decorator: d,
            mode: mode,
            history: sync::Mutex::new(vec![]),
            fn_timestamp: fn_timestamp,
        }
    }

    fn print_msg_header(&self,
                        io: &mut io::Write,
                        rd: &D::RecordDecorator,
                        info: &Record)
                        -> io::Result<()> {
        try!(rd.fmt_timestamp(io, &*self.fn_timestamp));
        try!(rd.fmt_level(io, &|io : &mut io::Write| write!(io, " {} ", info.level().as_short_str())));

        try!(rd.fmt_msg(io, &|io| write!(io, "{}", info.msg())));
        Ok(())
    }

    fn format_full(&self,
                   io: &mut io::Write,
                   info: &Record,
                   logger_values: &OwnedKeyValueList)
                   -> io::Result<()> {

        let r_decorator = self.decorator.decorate(info);


        try!(self.print_msg_header(io, &r_decorator, info));
        let mut serializer = Serializer::new(io, r_decorator);

        for (k, ref v) in logger_values.iter() {
            try!(serializer.print_comma());
            try!(v.serialize(info, k, &mut serializer));
        }

        for &(k, v) in info.values() {
            try!(serializer.print_comma());
            try!(v.serialize(info, k, &mut serializer));
        }
        let (mut io, _decorator_r) = serializer.finish();

        try!(write!(io, "\n"));

        Ok(())
    }


    fn format_compact(&self,
                      io: &mut io::Write,
                      info: &Record,
                      logger_values: &OwnedKeyValueList)
                      -> io::Result<()> {

        let r_decorator = self.decorator.decorate(info);
        let mut ser = Serializer::new(io, r_decorator);

        let indent = try!(self.format_recurse(&mut ser, info, logger_values));

        try!(self.print_indent(&mut ser.io, indent));

        try!(self.print_msg_header(&mut ser.io, &ser.decorator, info));

        for &(k, v) in info.values() {
            try!(ser.print_comma());
            try!(v.serialize(info, k, &mut ser));
        }
        try!(write!(&mut ser.io, "\n"));

        Ok(())
    }

    fn print_indent<W: io::Write>(&self, io: &mut W, indent: usize) -> io::Result<()> {
        for _ in 0..indent {
            try!(write!(io, "  "));
        }
        Ok(())
    }

    // record in the history, and check if should print
    // given set of values
    fn should_print(&self, address: usize, indent: usize) -> bool {
        let mut history = self.history.lock().unwrap();
        if history.len() <= indent {
            debug_assert_eq!(history.len(), indent);
            history.push(address);
            true
        } else {
            let should = history[indent] != address;
            history[indent] = address;
            should
        }
    }

    fn format_recurse<W: io::Write>(&self,
                                    ser: &mut Serializer<W, D::RecordDecorator>,
                                    info: &slog::Record,
                                    logger_values_ref: &slog::OwnedKeyValueList)
                                    -> io::Result<usize> {
        let mut indent = if logger_values_ref.parent().is_none() {
            0
        } else {
            try!(self.format_recurse(ser, info, logger_values_ref.parent().as_ref().unwrap()))
        };

        if let Some(logger_values) = logger_values_ref.values() {
            if self.should_print(logger_values_ref as *const _ as usize, indent) {
                try!(self.print_indent(&mut ser.io, indent));
                let mut clean = true;
                let mut logger_values = logger_values;
                loop {
                    let (k, ref v) = logger_values.head();
                    if !clean {
                        try!(ser.print_comma());
                    }
                    try!(v.serialize(info, k, ser));
                    clean = false;
                    logger_values = if let Some(v) = logger_values.tail() {
                        v
                    } else {
                        break;
                    }
                }
                try!(write!(&mut ser.io, "\n"));
            }
            indent += 1
        }

        Ok(indent)
    }
}

fn severity_to_color(lvl: Level) -> u8 {
    match lvl {
        Level::Critical => 5,
        Level::Error => 1,
        Level::Warning => 3,
        Level::Info => 2,
        Level::Debug => 6,
        Level::Trace => 4,
    }
}

/// Record decorator (color) for terminal output
pub struct ColorDecorator {
    use_color: bool,
}

impl ColorDecorator {
    /// New decorator that does color records
    pub fn new_colored() -> Self {
        ColorDecorator { use_color: true }
    }
    /// New decorator that does not color records
    pub fn new_plain() -> Self {
        ColorDecorator { use_color: true }
    }
}

/// Particular record decorator (color) for terminal output
pub struct ColorRecordDecorator {
    level_color: Option<u8>,
    key_bold: bool,
}


impl Decorator for ColorDecorator {
    type RecordDecorator = ColorRecordDecorator;

    fn decorate(&self, record: &Record) -> ColorRecordDecorator {
        if self.use_color {
            ColorRecordDecorator {
                level_color: Some(severity_to_color(record.level())),
                key_bold: true,
            }
        } else {
            ColorRecordDecorator {
                level_color: None,
                key_bold: false,
            }
        }
    }
}


impl RecordDecorator for ColorRecordDecorator {
    fn fmt_level(&self, io: &mut io::Write, f: &Fn(&mut io::Write) -> io::Result<()>) -> io::Result<()> {
        if let Some(level_color) = self.level_color {
            try!(write!(io, "\x1b[3{}m", level_color));
            try!(f(io));
            try!(write!(io, "\x1b[39m"));
        } else {
            try!(f(io));
        }
        Ok(())
    }


    fn fmt_msg(&self, io: &mut io::Write, f: &Fn(&mut io::Write) -> io::Result<()>) -> io::Result<()> {
        if self.key_bold {
            try!(write!(io, "\x1b[1m"));
            try!(f(io));
            try!(write!(io, "\x1b[0m"));
        } else {
            try!(f(io));
        }
        Ok(())
    }

    fn fmt_key(&self, io: &mut io::Write, f: &Fn(&mut io::Write) -> io::Result<()>) -> io::Result<()> {
        self.fmt_msg(io, f)
    }
}

struct Serializer<W, D: RecordDecorator> {
    io: W,
    decorator: D,
}

impl<W: io::Write, D: RecordDecorator> Serializer<W, D> {
    fn new(io: W, d: D) -> Self {
        Serializer {
            io: io,
            decorator: d,
        }
    }

    fn print_comma(&mut self) -> io::Result<()> {
        try!(self.decorator.fmt_separator(&mut self.io, &|io : &mut io::Write| write!(io, ", ")));
        Ok(())
    }

    fn finish(self) -> (W, D) {
        (self.io, self.decorator)
    }
}

macro_rules! s(
    ($s:expr, $k:expr, $v:expr) => {
        try!($s.decorator.fmt_key(&mut $s.io, &|io : &mut io::Write| write!(io, "{}", $k)));
        try!($s.decorator.fmt_separator(&mut $s.io, &|io : &mut io::Write| write!(io, ": ")));
        try!($s.decorator.fmt_value(&mut $s.io, &|io : &mut io::Write| write!(io, "{}", $v)));
    };
);


impl<W: io::Write, D: RecordDecorator> slog::ser::Serializer for Serializer<W, D> {
    fn emit_none(&mut self, key: &str) -> ser::Result {
        s!(self, key, "None");
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> ser::Result {
        s!(self, key, "()");
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {
        s!(self, key, val);
        Ok(())
    }

}

impl<D: Decorator + Send + Sync> StreamFormat for Format<D> {
    fn format(&self,
              io: &mut io::Write,
              info: &Record,
              logger_values: &OwnedKeyValueList)
              -> io::Result<()> {
        match self.mode {
            FormatMode::Compact => self.format_compact(io, info, logger_values),
            FormatMode::Full => self.format_full(io, info, logger_values),
        }
    }
}

const TIMESTAMP_FORMAT: &'static str = "%b %d %H:%M:%S%.3f";

fn timestamp_local(io : &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::Local::now().format(TIMESTAMP_FORMAT))
}

fn timestamp_utc(io : &mut io::Write) -> io::Result<()> {
    write!(io, "{}", chrono::UTC::now().format(TIMESTAMP_FORMAT))
}

/// Streamer builder
pub struct StreamerBuilder {
    color: Option<bool>, // None = auto
    stdout: bool,
    async: bool,
    mode: FormatMode,
    fn_timestamp: Box<TimestampFn>,
}

impl StreamerBuilder {
    /// New `StreamerBuilder`
    pub fn new() -> Self {
        StreamerBuilder {
            color: None,
            stdout: true,
            async: false,
            mode: FormatMode::Full,
            fn_timestamp: Box::new(timestamp_local),
        }
    }

    /// Force colored output
    pub fn color(mut self) -> Self {
        self.color = Some(true);
        self
    }

    /// Force plain output
    pub fn plain(mut self) -> Self {
        self.color = Some(false);
        self
    }

    /// Auto detect color (default)
    pub fn auto_color(mut self) -> Self {
        self.color = None;
        self
    }

    /// Output to stderr
    pub fn stderr(mut self) -> Self {
        self.stdout = false;
        self
    }

    /// Output to stdout (default)
    pub fn stdout(mut self) -> Self {
        self.stdout = true;
        self
    }

    /// Output using full mode
    pub fn full(mut self) -> Self {
        self.mode = FormatMode::Full;
        self
    }

    /// Output using compact mode (default)
    pub fn compact(mut self) -> Self {
        self.mode = FormatMode::Compact;
        self
    }

    /// Use asynchronous streamer
    pub fn async(mut self) -> Self {
        self.async = true;
        self
    }

    /// Use synchronous streamer (default)
    pub fn sync(mut self) -> Self {
        self.async = false;
        self
    }

    /// Use the UTC time zone for the timestamp
    pub fn use_utc_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_utc);
        self
    }

    /// Use the local time zone for the timestamp (default)
    pub fn use_local_timestamp(mut self) -> Self {
        self.fn_timestamp = Box::new(timestamp_local);
        self
    }

    /// Provide a custom function to generate the timestamp
    pub fn use_custom_timestamp<F>(mut self, f: F) -> Self
        where F : Fn(&mut io::Write) -> io::Result<()> + 'static + Send + Sync {
        self.fn_timestamp = Box::new(f);
        self
    }

    /// Build the streamer
    pub fn build(self) -> Box<slog::Drain<Error=io::Error>> {
        let color = self.color.unwrap_or(if self.stdout {
            stdout_isatty()
        } else {
            stderr_isatty()
        });

        let format = Format::new(
            self.mode,
            ColorDecorator { use_color: color },
            self.fn_timestamp
        );

        let io = if self.stdout {
            Box::new(io::stdout()) as Box<io::Write + Send>
        } else {
            Box::new(io::stderr()) as Box<io::Write + Send>
        };

        if self.async {
            Box::new(async_stream(io, format))
        } else {
            Box::new(stream(io, format))
        }
    }
}

impl Default for StreamerBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Build `slog_stream::Streamer`/`slog_stream::AsyncStreamer` that
/// will output logging records to stderr/stderr.
pub fn streamer() -> StreamerBuilder {
    StreamerBuilder::new()
}
