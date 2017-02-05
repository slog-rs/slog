
#[derive(Debug)]
#[cfg(feature = "std")]
/// Serialization Error
pub enum Error {
    /// `io::Error` (no available in ![no_std] mode)
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
    /// `fmt::Error`
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
        /// Emit $t
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

    /// Emit usize
    impl_default_as_fmt!(usize, emit_usize);
    /// Emit isize
    impl_default_as_fmt!(isize, emit_isize);
    /// Emit bool
    impl_default_as_fmt!(bool, emit_bool);
    /// Emit char
    impl_default_as_fmt!(char, emit_char);
    /// Emit u8
    impl_default_as_fmt!(u8, emit_u8);
    /// Emit i8
    impl_default_as_fmt!(i8, emit_i8);
    /// Emit u16
    impl_default_as_fmt!(u16, emit_u16);
    /// Emit i16
    impl_default_as_fmt!(i16, emit_i16);
    /// Emit u32
    impl_default_as_fmt!(u32, emit_u32);
    /// Emit i32
    impl_default_as_fmt!(i32, emit_i32);
    /// Emit f32
    impl_default_as_fmt!(f32, emit_f32);
    /// Emit u64
    impl_default_as_fmt!(u64, emit_u64);
    /// Emit i64
    impl_default_as_fmt!(i64, emit_i64);
    /// Emit f64
    impl_default_as_fmt!(f64, emit_f64);
    /// Emit str
    impl_default_as_fmt!(&str, emit_str);

    /// Emit `()`
    fn emit_unit(&mut self, key: &str) -> Result {
        self.emit_arguments(key, &format_args!("()"))
    }

    /// Emit `None`
    fn emit_none(&mut self, key: &str) -> Result {
        self.emit_arguments(key, &format_args!(""))
    }

    /// Emit `Some`
    fn emit_some(&mut self, record : &Record, key: &str, val : &Value) -> Result {
        /// All this trickery is required because here
        /// `self` is a concrete type, and not a trait object anymore.
        ///
        /// Simpler way would requre adding `Serialized : Sized`.
        ///
        /// If you know of any method to simplify it, PR would be very
        /// welcome.
        struct Wrap<F>(F)
        where F : for<'c, 'd> FnMut(&'c str, &'c fmt::Arguments<'d>) -> Result ;

        impl<F> Serializer for Wrap<F>
        where F : for<'c, 'd> FnMut(&'c str, &'c fmt::Arguments<'d>) -> Result
        {
            fn emit_some(&mut self, record : &Record, key: &str, val : &Value) -> Result {
                val.serialize(record, key, self)
            }

            fn emit_arguments(&mut self, key: &str, val: &fmt::Arguments) -> Result {
                (self.0)(key, val)
            }
        }

        let mut s = Wrap(|key, fmt| {
            self.emit_arguments(key, fmt)
        });

        val.serialize(record, key, &mut s)
    }


    /// Emit `fmt::Arguments`
    ///
    /// This is the only method that has to implemented, but for
    /// performance and type-retaining reason most serious `Serializer`s
    /// will want to implement all other methods as well.
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
