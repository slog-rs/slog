use serde;
use std::fmt::Write;

/// Value that can be serialized
///
/// Loggers require values in key-value pairs to
/// implement this trait.
///
pub trait Serialize : Send+Sync+'static {
    /// Serialize self into `Serializer`
    ///
    /// Structs implementing this trait should generally
    /// only call respective methods of `serializer`.
    fn serialize(&self, key : &str, serializer : &mut Serializer);
}

/// Value that can be serialized and stored
/// in logger itself.
///
/// As Loggers itself must be thread-safe, they can only
/// store values implementing this trait.
pub trait SyncSerialize : Send+Sync+'static+Serialize {
}


/// Serializer
///
/// Drains using `Format` will internally use
/// types implementing this trait.
pub trait Serializer {
    /// Emit u32
    fn emit_u32(&mut self, key : &str, val : u32);
    /// Emit i32
    fn emit_i32(&mut self, key : &str, val : i32);
    /// Emit u64
    fn emit_u64(&mut self, key : &str, val : u64);
    /// Emit i64
    fn emit_i64(&mut self, key : &str, val : i64);
    /// Emit usize
    fn emit_usize(&mut self, key : &str, val : usize);
    /// Emit isize
    fn emit_isize(&mut self, key : &str, val : isize);
    /// Emit str
    fn emit_str(&mut self, key : &str, val : &str);
}

impl Serialize for str {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self)
    }
}


impl Serialize for String {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self.as_str())
    }
}

impl Serialize for Box<String> {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        serializer.emit_str(key, self.as_str())
    }
}

impl SyncSerialize for Box<String> { }


#[macro_export]
macro_rules! impl_serialize_for{
    ($t:ty, $f:ident) => {
        impl Serialize for $t {
            fn serialize(&self, key : &str, serializer : &mut Serializer) {
                serializer.$f(key, *self)
            }
        }

        impl Serialize for Box<$t> where $t : Send+Sync {
            fn serialize(&self, key : &str, serializer : &mut Serializer) {
                serializer.$f(key, **self)
            }
        }

        impl SyncSerialize for Box<$t> where $t : Send+Sync { }


    };
}

//impl_serialize_for!(str, emit_str);
impl_serialize_for!(&'static str, emit_str);
impl_serialize_for!(usize, emit_usize);
impl_serialize_for!(isize, emit_isize);
impl_serialize_for!(u32, emit_u32);
impl_serialize_for!(i32, emit_i32);
impl_serialize_for!(u64, emit_u64);
impl_serialize_for!(i64, emit_i64);

impl<S : Serialize, F : 'static+Sync+Send+Fn()->S> Serialize for Box<F> {
    fn serialize(&self, key : &str, serializer : &mut Serializer) {
        (*self)().serialize(key, serializer)
    }
}

impl<S : Serialize, F : 'static+Sync+Send+Fn()->S> SyncSerialize for Box<F> { }


/// Newtype to wrap serde Serializer, so that `Serialize` can be implemented
/// for it
pub struct SerdeSerializer<'a, S: 'a+serde::Serializer>(pub &'a mut S);

impl<'a, S> Serializer for SerdeSerializer<'a, S> where S : 'a+serde::Serializer {
    fn emit_usize(&mut self, key : &str, val : usize) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
    fn emit_isize(&mut self, key : &str, val : isize) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
    fn emit_u32(&mut self, key : &str, val : u32) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
    fn emit_i32(&mut self, key : &str, val : i32) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
    fn emit_u64(&mut self, key : &str, val : u64) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
    fn emit_i64(&mut self, key : &str, val : i64) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }

    fn emit_str(&mut self, key : &str, val : &str) {
        let _ = serde::Serializer::serialize_map_elt(self.0, key, val);
    }
}

impl Serializer for String {
    fn emit_usize(&mut self, key : &str, val : usize) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_isize(&mut self, key : &str, val : isize) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_u32(&mut self, key : &str, val : u32) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i32(&mut self, key : &str, val : i32) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_u64(&mut self, key : &str, val : u64) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_i64(&mut self, key : &str, val : i64) {
        write!(self, "{}: {}", key, val).unwrap()
    }
    fn emit_str(&mut self, key : &str, val : &str) {
        write!(self, "{}: {}", key, val).unwrap()
    }
}
