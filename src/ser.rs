use std::io;
use serialize::hex::ToHex;

use super::logger::{RecordInfo};

#[allow(missing_docs)]
mod error {
    use std::io;

    error_chain! {
        types {
            Error, ErrorKind, ChainErr, Result;
        }
        links {}
        foreign_links {
            io::Error, Io, "io error";
        }
        errors {
            Other {
                description("other serialization error")
                    display("other serialization error")
            }
        }
    }
}

pub use self::error::{Error, Result, ErrorKind};

/// Value that can be serialized
///
/// Loggers require values in key-value pairs to
/// implement this trait.
///
pub trait Serialize: Send + Sync {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> ;
}

/// Value that can be serialized and stored
/// in logger itself.
///
/// As Loggers itself must be thread-safe, they can only
/// store values implementing this trait.
pub trait SyncSerialize: Send + Sync + 'static + Serialize {}


/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {
    /// Emit bool
    fn emit_bool(&mut self, key: &str, val: bool) -> Result<()>;
    /// Emit `()`
    fn emit_unit(&mut self, key: &str) -> Result<()>;
    /// Emit `None`
    fn emit_none(&mut self, key: &str) -> Result<()>;
    /// Emit char
    fn emit_char(&mut self, key: &str, val: char) -> Result<()>;
    /// Emit bytes
    fn emit_bytes(&mut self, key: &str, val: &[u8]) -> Result<()>;
    /// Emit u8
    fn emit_u8(&mut self, key: &str, val: u8) -> Result<()>;
    /// Emit i8
    fn emit_i8(&mut self, key: &str, val: i8) -> Result<()>;
    /// Emit u16
    fn emit_u16(&mut self, key: &str, val: u16) -> Result<()>;
    /// Emit i16
    fn emit_i16(&mut self, key: &str, val: i16) -> Result<()>;
    /// Emit u32
    fn emit_u32(&mut self, key: &str, val: u32) -> Result<()>;
    /// Emit i32
    fn emit_i32(&mut self, key: &str, val: i32) -> Result<()>;
    /// Emit f32
    fn emit_f32(&mut self, key: &str, val: f32) -> Result<()>;
    /// Emit u64
    fn emit_u64(&mut self, key: &str, val: u64) -> Result<()>;
    /// Emit i64
    fn emit_i64(&mut self, key: &str, val: i64) -> Result<()>;
    /// Emit f64
    fn emit_f64(&mut self, key: &str, val: f64) -> Result<()>;
    /// Emit usize
    fn emit_usize(&mut self, key: &str, val: usize) -> Result<()>;
    /// Emit isize
    fn emit_isize(&mut self, key: &str, val: isize) -> Result<()>;
    /// Emit str
    fn emit_str(&mut self, key: &str, val: &str) -> Result<()>;
}

impl Serialize for str {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        serializer.emit_str(key, self)
    }
}

impl<'a> Serialize for &'a str {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        serializer.emit_str(key, self)
    }
}

impl SyncSerialize for &'static str {}

impl Serialize for [u8] {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        serializer.emit_bytes(key, self)
    }
}

impl SyncSerialize for [u8] {}

impl Serialize for Vec<u8> {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        serializer.emit_bytes(key, self.as_slice())
    }
}

impl SyncSerialize for Vec<u8> {}


impl<T: Serialize> Serialize for Option<T> {
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        match *self {
            Some(ref s) => s.serialize(rinfo, key, serializer),
            None => serializer.emit_none(key),
        }
    }
}

impl<T: Serialize + 'static> SyncSerialize for Option<T> {}

impl Serialize for String {
    fn serialize(&self, _rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        serializer.emit_str(key, self.as_str())
    }
}

impl SyncSerialize for String {}


macro_rules! impl_serialize_for{
    ($t:ty, $f:ident) => {
        impl Serialize for $t {
            fn serialize(&self, _rinfo : &RecordInfo, key : &str, serializer : &mut Serializer)
                         -> Result<()> {
                serializer.$f(key, *self)
            }
        }

        impl SyncSerialize for $t where $t : Send+Sync+'static { }
    };
}

impl_serialize_for!(usize, emit_usize);
impl_serialize_for!(isize, emit_isize);
impl_serialize_for!(bool, emit_bool);
impl_serialize_for!(char, emit_char);
impl_serialize_for!(u8, emit_u8);
impl_serialize_for!(i8, emit_i8);
impl_serialize_for!(u16, emit_u16);
impl_serialize_for!(i16, emit_i16);
impl_serialize_for!(u32, emit_u32);
impl_serialize_for!(i32, emit_i32);
impl_serialize_for!(f32, emit_f32);
impl_serialize_for!(u64, emit_u64);
impl_serialize_for!(i64, emit_i64);
impl_serialize_for!(f64, emit_f64);

impl<S: 'static + Serialize, F: Sync + Send + for<'c> Fn(&'c RecordInfo<'c>) -> S> Serialize for F {
    fn serialize(&self, rinfo: &RecordInfo, key: &str, serializer: &mut Serializer) -> Result<()> {
        (*self)(&rinfo).serialize(rinfo, key, serializer)
    }
}

impl<S: 'static + Serialize, F: 'static + Sync + Send + for<'c, 'd> Fn(&'c RecordInfo<'d>) -> S> SyncSerialize for F {}


impl<W: io::Write + ?Sized> Serializer for W {
    fn emit_none(&mut self, key: &str) -> Result<()> {
        try!(write!(self, "{}: {}", key, "None"));
        Ok(())
    }
    fn emit_unit(&mut self, key: &str) -> Result<()> {
        try!(write!(self, "{}: ()", key, ));
        Ok(())
    }

    fn emit_bool(&mut self, key: &str, val: bool) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }

    fn emit_char(&mut self, key: &str, val: char) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_bytes(&mut self, key: &str, val: &[u8]) -> Result<()> {
        try!(write!(self, "{}: {}", key, val.to_hex()));
        Ok(())
    }

    fn emit_usize(&mut self, key: &str, val: usize) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_isize(&mut self, key: &str, val: isize) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }

    fn emit_u8(&mut self, key: &str, val: u8) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i8(&mut self, key: &str, val: i8) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u16(&mut self, key: &str, val: u16) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i16(&mut self, key: &str, val: i16) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u32(&mut self, key: &str, val: u32) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i32(&mut self, key: &str, val: i32) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_f32(&mut self, key: &str, val: f32) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_u64(&mut self, key: &str, val: u64) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_i64(&mut self, key: &str, val: i64) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_f64(&mut self, key: &str, val: f64) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
    fn emit_str(&mut self, key: &str, val: &str) -> Result<()> {
        try!(write!(self, "{}: {}", key, val));
        Ok(())
    }
}
