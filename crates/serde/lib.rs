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

thread_local! {
    static TL_BUF: RefCell<String> = RefCell::new(String::with_capacity(128))
}

/// slog-rs's `Serializer` adapter for `serde::Serializer`
///
/// Newtype to wrap serde Serializer, so that `Serialize` can be implemented
/// for it
pub struct SerdeSerializer<'a, S: 'a + serde::Serializer>(pub &'a mut S);

impl<'a, S> slog::ser::Serializer for SerdeSerializer<'a, S>
    where S: 'a + serde::Serializer
{
    fn emit_bool(&mut self, key: &str, val: bool) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }

    fn emit_unit(&mut self, key: &str) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, ())
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }

    fn emit_char(&mut self, key: &str, val: char) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }

    fn emit_none(&mut self, key: &str) -> ser::Result {
        let none: Option<()> = None;
        serde::Serializer::serialize_map_elt(self.0, key, none)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_usize(&mut self, key: &str, val: usize) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> ser::Result {
        serde::Serializer::serialize_map_elt(self.0, key, val)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> ser::Result {

        TL_BUF.with(|buf| {
            let mut buf = buf.borrow_mut();

            buf.write_fmt(*val).unwrap();

            let res = {
                || {
                    serde::Serializer::serialize_map_elt(self.0, key, &*buf)
                        .map_err(|_| io::Error::new(io::ErrorKind::Other, "serde serialization error").into())

                }
            }();
            buf.clear();
            res
        })
    }
}
