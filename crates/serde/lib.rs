//! Serde serialization adapter for slog-rs
//!
//! Utility crate useful when building own `slog-rs` `Drain` trait
//! implementation utilizing `serde` serialization.

#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate serde;

use std::{io, fmt};
use slog::ser;
use std::cell::RefCell;
use std::fmt::Write;
use std::result;

thread_local! {
    static TL_BUF: RefCell<String> = RefCell::new(String::with_capacity(128))
}

/// slog-rs's `Serializer` adapter for `serde::Serializer`
///
/// Newtype to wrap serde Serializer, so that `Serialize` can be implemented
/// for it
pub struct SerdeSerializer<S: serde::Serializer>{
    /// Reference to underlying `serde::Serializer`
    ser : S,
    /// Current state of map serializing: `serde::Seriaizer::MapState`
    map_state : S::MapState,
}

impl<S: serde::Serializer> SerdeSerializer<S> {

    /// Start serializing map of values
    pub fn start(mut ser : S, len: Option<usize>) -> result::Result<Self, ser::Error> {
        let map_state = try!(
            ser.serialize_map(len)
                .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error"))
        );
        Ok(SerdeSerializer {
            ser: ser,
            map_state: map_state,
        })
    }

    /// Finish serialization, and return the serializer
    pub fn end(mut self) -> (S, ser::Result) {
        let res = self.ser.serialize_map_end(self.map_state)
             .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into());

        (self.ser, res)
    }
}

macro_rules! impl_m(
    ($s:expr, $key:expr, $val:expr) => ({
        try!(serde::Serializer::serialize_map_key(&mut $s.ser, &mut $s.map_state, $key)
             .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error")));
       serde::Serializer::serialize_map_value(&mut $s.ser, &mut $s.map_state, $val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    });
);

impl<S> slog::ser::Serializer for SerdeSerializer<S>
    where S: serde::Serializer
{
    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        impl_m!(self, key, val)
    }

    fn emit_unit(&mut self, key: &str) -> ser::Result {
        impl_m!(self, key, ())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        impl_m!(self, key, val)
    }

    fn emit_none(&mut self, key: &str) -> ser::Result {
        let val: Option<()> = None;
        impl_m!(self, key, val)
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        impl_m!(self, key, val)
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {

        TL_BUF.with(|buf| {
            let mut buf = buf.borrow_mut();

            buf.write_fmt(*val).unwrap();

            let res = {
                || {
                    impl_m!(self, key, &*buf)
                }
            }();
            buf.clear();
            res
        })
    }
}
