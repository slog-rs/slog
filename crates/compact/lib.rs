//! Compact textual format for slog-rs
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
use std::{io, sync};

/// Serializer used by `Format`
struct Serializer<W> {
    io: W,
}

impl<W: io::Write> Serializer<W> {
    fn new(io: W) -> Self {
        Serializer {
            io: io,
        }
    }
}

impl<W: io::Write> slog::ser::Serializer for Serializer<W> {
    fn emit_none(&mut self, key: &str) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, "None"));
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: ()", key));
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> slog::ser::Result<()> {
        try!(write!(self.io, "{}: {}", key, val));
        Ok(())
    }
}

/// Compact logging format
pub struct Format {
#[allow(unused)]
    history : sync::Mutex<Vec<usize>>,
}

impl Format {
    /// New `Format`
    pub fn new() -> Self {
        Format {
            history : sync::Mutex::new(vec!()),
        }
    }

    fn print_indent<W: io::Write>(
        &self,
        io: &mut W,
        indent : usize) -> io::Result<()> {
        for _ in 0..indent {
            try!(write!(io, "  "));
        }
        Ok(())
    }

    // record in the history, and check if should print
    // given set of values
    fn should_print(&self, address : usize, indent : usize) -> bool {
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
                                     ser : &mut Serializer<W>,
                                     info: &slog::Record,
                                     logger_values: &slog::OwnedKeyValueList)
                                    -> slog::format::Result<usize> {
        let mut indent = if logger_values.parent().is_none() {
            0
        } else {
            try!(self.format_recurse(ser, info, logger_values.parent().as_ref().unwrap()))
        };

        if !logger_values.values().is_empty() {
            if self.should_print(logger_values.values() as *const _ as usize, indent) {
                try!(self.print_indent(&mut ser.io, indent));
                let mut clean = true;
                for &(ref k, ref v) in logger_values.values() {
                    if !clean {
                        try!(write!(&mut ser.io, ", "));
                    }
                    try!(v.serialize(info, k, ser));
                    clean = false;
                }
                try!(write!(&mut ser.io, "\n"));
            }
            indent += 1
        }

        Ok(indent)
    }
}


impl slog::format::Format for Format {
    fn format(&self,
              io: &mut io::Write,
              info: &slog::Record,
              logger_values: &slog::OwnedKeyValueList)
              -> slog::format::Result<()> {

        let mut ser = Serializer::new(io);

        let mut indent = try!(self.format_recurse(&mut ser, info, logger_values));

        try!(self.print_indent(&mut ser.io, indent));
        let _ = try!(write!(&mut ser.io,
                            "{} {} {}",
                            info.ts().format("%b %d %H:%M:%S%.3f"),
                            info.level().as_short_str(),
                            info.msg()));

        for &(k, v) in info.values() {
            try!(write!(&mut ser.io, ", "));
            try!(v.serialize(info, k, &mut ser));
        }
        try!(write!(&mut ser.io, "\n"));

        Ok(())
    }
}
