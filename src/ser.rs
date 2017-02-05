
#[cfg(feature = "std")]
use std;
use core;
use core::{result, fmt};

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::arc::Arc;

#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;

#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;


#[cfg(feature = "std")]
use std::string::String;
#[cfg(not(feature = "std"))]
use collections::string::String;

use super::Record;

#[derive(Debug)]
#[cfg(feature = "std")]
/// Serialization Error
pub enum Error {
    /// `io::Error`
    Io(std::io::Error),
    /// `fmt::Error`
    Fmt(std::fmt::Error),
    /// Other error
    Other,
}

#[derive(Debug)]
#[cfg(not(feature = "std"))]
/// Serialization Error
pub enum Error {
    Fmt(core::fmt::Error),
    /// Other error
    Other,
}

/// Serialization `Result`
pub type Result = result::Result<(), Error>;

#[cfg(feature = "std")]
impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Error {
        Error::Io(err)
    }
}

impl From<core::fmt::Error> for Error {
    fn from(_: core::fmt::Error) -> Error {
        Error::Other
    }
}

#[cfg(feature = "std")]
impl From<Error> for std::io::Error {
    fn from(e: Error) -> std::io::Error {
        match e {
            Error::Io(e) => e,
            Error::Fmt(_) => std::io::Error::new(std::io::ErrorKind::Other, "formatting error"),
            Error::Other => std::io::Error::new(std::io::ErrorKind::Other, "other error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Io(ref e) => e.description(),
            Error::Fmt(_) => "formatting error",
            Error::Other => "serialization error",
        }
    }

    fn cause(&self) -> Option<&std::error::Error> {
        match *self {
            Error::Io(ref e) => Some(e),
            Error::Fmt(ref e) => Some(e),
            Error::Other => None,
        }
    }
}

#[cfg(feature = "std")]
impl core::fmt::Display for Error {
    fn fmt(&self, fmt: &mut core::fmt::Formatter) -> std::fmt::Result {
        match *self {
            Error::Io(ref e) => e.fmt(fmt),
            Error::Fmt(ref e) => e.fmt(fmt),
            Error::Other => fmt.write_str("Other serialization error"),
        }
    }
}

macro_rules! impl_default_as_fmt{
    ($t:ty, $f:ident) => {
        fn $f(&mut self, key : &str, val : $t)
            -> Result {
                self.emit_arguments(key, &format_args!("{}", val))
            }
    };
}

/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {

    /// Emit bool
    impl_default_as_fmt!(bool, emit_bool);

    /// Emit `()`
    fn emit_unit(&mut self, key: &str) -> Result;
    /// Emit `None`
    fn emit_none(&mut self, key: &str) -> Result;
    /// Emit char
    fn emit_char(&mut self, key: &str, val: char) -> Result;
    /// Emit u8
    fn emit_u8(&mut self, key: &str, val: u8) -> Result;
    /// Emit i8
    fn emit_i8(&mut self, key: &str, val: i8) -> Result;
    /// Emit u16
    fn emit_u16(&mut self, key: &str, val: u16) -> Result;
    /// Emit i16
    fn emit_i16(&mut self, key: &str, val: i16) -> Result;
    /// Emit u32
    fn emit_u32(&mut self, key: &str, val: u32) -> Result;
    /// Emit i32
    fn emit_i32(&mut self, key: &str, val: i32) -> Result;
    /// Emit f32
    fn emit_f32(&mut self, key: &str, val: f32) -> Result;
    /// Emit u64
    fn emit_u64(&mut self, key: &str, val: u64) -> Result;
    /// Emit i64
    fn emit_i64(&mut self, key: &str, val: i64) -> Result;
    /// Emit f64
    fn emit_f64(&mut self, key: &str, val: f64) -> Result;
    /// Emit usize
    fn emit_usize(&mut self, key: &str, val: usize) -> Result;
    /// Emit isize
    fn emit_isize(&mut self, key: &str, val: isize) -> Result;
    /// Emit str
    fn emit_str(&mut self, key: &str, val: &str) -> Result;
    /// Emit `fmt::Arguments`
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> Result;
}

/// Serializer that formats all arguments as strings
/// and passes them to given function.
pub struct AsStrSerializer<F>(pub F)
    where F : for <'a, 'b> FnMut(&'b str, fmt::Arguments<'a>) -> Result;

macro_rules! impl_as_str_emit{
    ($t:ty, $f:ident) => {
        fn $f(&mut self, key : &str, val : $t)
            -> result::Result<(), Error> {
                (self.0)(key, format_args!("{}", val))
            }
    };
}

impl<F> Serializer for AsStrSerializer<F> 
    where F : for <'a, 'b> FnMut(&'b str, fmt::Arguments<'a>) -> Result
{
    impl_as_str_emit!(usize, emit_usize);
    impl_as_str_emit!(isize, emit_isize);
    impl_as_str_emit!(bool, emit_bool);
    impl_as_str_emit!(char, emit_char);
    impl_as_str_emit!(u8, emit_u8);
    impl_as_str_emit!(i8, emit_i8);
    impl_as_str_emit!(u16, emit_u16);
    impl_as_str_emit!(i16, emit_i16);
    impl_as_str_emit!(u32, emit_u32);
    impl_as_str_emit!(i32, emit_i32);
    impl_as_str_emit!(f32, emit_f32);
    impl_as_str_emit!(u64, emit_u64);
    impl_as_str_emit!(i64, emit_i64);
    impl_as_str_emit!(f64, emit_f64);
    impl_as_str_emit!(&str, emit_str);


    fn emit_unit(&mut self, key: &str) -> Result {
        (self.0)(key, format_args!("()"))
    }
    fn emit_none(&mut self, key: &str) -> Result {
        (self.0)(key, format_args!(""))
    }
    fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> Result {
        (self.0)(key, *val)
    }
}

/// Value that can be serialized
pub trait Value {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error>;
}

/// Value that can be serialized
pub trait Key {
    /// To text representation
    fn as_str(&self) -> &str;
}

impl<'a> Key for &'a str {
    fn as_str(&self) -> &str {
        &self
    }
}

/// `Value` that is also `Send+Sync`
///
/// TODO: Consider removing
pub trait SyncValue: Send + Sync + 'static + Value {}

impl<V> SyncValue for V
where V : Value + Send + Sync + 'static {
}

macro_rules! impl_value_for{
    ($t:ty, $f:ident) => {
        impl Value for $t {
            fn serialize(&self, _record : &Record, key : &str, serializer : &mut Serializer)
                         -> result::Result<(), Error> {
                serializer.$f(key, *self)
            }
        }
    };
}

impl_value_for!(usize, emit_usize);
impl_value_for!(isize, emit_isize);
impl_value_for!(bool, emit_bool);
impl_value_for!(char, emit_char);
impl_value_for!(u8, emit_u8);
impl_value_for!(i8, emit_i8);
impl_value_for!(u16, emit_u16);
impl_value_for!(i16, emit_i16);
impl_value_for!(u32, emit_u32);
impl_value_for!(i32, emit_i32);
impl_value_for!(f32, emit_f32);
impl_value_for!(u64, emit_u64);
impl_value_for!(i64, emit_i64);
impl_value_for!(f64, emit_f64);

impl Value for () {
    fn serialize(&self,
                 _record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        serializer.emit_unit(key)
    }
}


impl Value for str {
    fn serialize(&self,
                 _record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        serializer.emit_str(key, self)
    }
}

impl<'a> Value for &'a str {
    fn serialize(&self,
                 _record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        serializer.emit_str(key, self)
    }
}

impl<'a> Value for fmt::Arguments<'a> {
    fn serialize(&self,
                 _record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        serializer.emit_arguments(key, self)
    }
}

impl Value for String {
    fn serialize(&self,
                 _record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        serializer.emit_str(key, self.as_str())
    }
}

impl<T: Value> Value for Option<T> {
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        match *self {
            Some(ref s) => s.serialize(record, key, serializer),
            None => serializer.emit_none(key),
        }
    }
}

impl Value for Box<Value + Send + 'static> {
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Value for Arc<T>
    where T: Value
{
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Value for Rc<T>
    where T: Value
{
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        (**self).serialize(record, key, serializer)
    }
}

impl<T> Value for core::num::Wrapping<T>
    where T: Value
{
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        self.0.serialize(record, key, serializer)
    }
}

impl<S: 'static + Value, F> Value for F
    where F: 'static + for<'c, 'd> Fn(&'c Record<'d>) -> S
{
    fn serialize(&self,
                 record: &Record,
                 key: &str,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error> {
        (*self)(record).serialize(record, key, serializer)
    }
}

/// It's more natural for closures used as lazy values to return `Serialize`
/// implementing type, but sometimes that forces an allocation (eg. Strings)
///
/// In some cases it might make sense for another closure form to be used - one
/// taking a serializer as an argument, which avoids lifetimes / allocation issues.
///
/// Unfortunately to avoid closure traits ambiguity, as `&Fn` has to be used.
///
/// Generally this method should be used if it avoids a big allocation of
/// `Serialize`-implementing type in performance-critical logging statement.
///
/// TODO: Move examples from tests
pub struct ValueSerializer<'a> {
    record: &'a Record<'a>,
    key: &'a str,
    serializer: &'a mut Serializer,
    done: bool,
}

impl<'a> ValueSerializer<'a> {
    /// Serialize a value
    ///
    /// This consumes `self` to prevent serializing one value multiple times
    pub fn serialize<'b, S: 'b + Value>(mut self, s: S) -> result::Result<(), Error> {
        self.done = true;
        s.serialize(self.record, self.key, self.serializer)
    }
}

impl<'a> Drop for ValueSerializer<'a> {
    fn drop(&mut self) {
        if !self.done {
            // unfortunately this gives no change to return serialization errors
            let _ = self.serializer.emit_unit(self.key);
        }
    }
}

impl<'a> Value for &'a for <'c, 'd> Fn(&'c Record<'d>, ValueSerializer<'c>) -> result::Result<(), Error> {
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer)
        -> result::Result<(), Error> {
        let ser = ValueSerializer {
            record: record,
            key: key,
            serializer: serializer,
            done: false,
        };
        (self)(record, ser)
    }
}

impl Value for Box<for <'c, 'd> Fn(&'c Record<'d>, ValueSerializer<'c>) -> result::Result<(), Error>> {
    fn serialize(&self, record: &Record, key: &str, serializer: &mut Serializer)
        -> result::Result<(), Error> {
        let ser = ValueSerializer {
            record: record,
            key: key,
            serializer: serializer,
            done: false,
        };
        (self)(record, ser)
    }
}

/// Key-value pair(s) that can be serialized
///
/// Zero, one or more key value pairs chained together
pub trait KV {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self,
                 record: &Record,
                 serializer: &mut Serializer)
                 -> result::Result<(), Error>;

    /// Split into tuple of `(first, rest)`
    fn split_first(&self) -> Option<(&KV, &KV)>;
}

/// Single pair `Key` and `Value`
pub struct SingleKV<K, V>(pub K, pub V)
    where K : Key, V : Value;

static STATIC_TERMINATOR_UNIT : () = ();

impl<K, V> KV for SingleKV<K, V>
    where K : Key,
          V : Value
{
    fn serialize(&self,
                 record: &Record,
                 serializer: &mut Serializer)
        -> result::Result<(), Error> {
            self.1.serialize(record, self.0.as_str(), serializer)
        }

    fn split_first(&self) -> Option<(&KV, &KV)> {
        Some((self, &STATIC_TERMINATOR_UNIT))
    }
}

impl KV for () {
    fn serialize(&self,
                 _record: &Record,
                 _serializer: &mut Serializer)
        -> result::Result<(), Error> {
            Ok(())
        }

    fn split_first(&self) -> Option<(&KV, &KV)> {
        None
    }
}

impl<T: KV, R: KV> KV for (T, R) {
    fn serialize(&self,
                 record: &Record,
                 serializer: &mut Serializer)
        -> result::Result<(), Error> {
            try!(self.0.serialize(record, serializer));
            self.1.serialize(record, serializer)
        }


    fn split_first(&self) -> Option<(&KV, &KV)> {
        Some((&self.0, &self.1))
    }
}

impl<T> SyncKV for T
where T : KV+Sync+Send+'static {}

/// Key-value pair that is `Sync` and `Send` and thus
/// can stored as part of `Logger` itself.
///
/// As Loggers itself must be thread-safe, they can only
/// store values implementing this trait.
///
/// TODO: Consider removing
pub trait SyncKV: Send + Sync + 'static + KV {}
pub type SyncMultiKV = SyncKV;


