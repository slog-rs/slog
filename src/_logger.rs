
/// Logger
///
/// Loggers are thread-safe and reference counted, so can be freely
/// passed around the code.
///
/// Each logger is built with a set of key-values.
///
/// Child loggers are build from existing loggers, and copy
/// all the key-values from their parents
///
/// Loggers form hierarchies sharing a drain. Setting a drain on
/// any logger will change it on all loggers in given hierarchy.
#[derive(Clone)]
pub struct Logger {
    drain: Arc<Drain<Error=()>>,
    values: Arc<OwnedKeyValueList>,
}

/// Log Record message
pub trait Message {
    /// Take a string
    fn as_str(&self) -> Cow<str>;

    /// Write it into `io` (which might be fast than `as_str()` it does not
    /// need to allocate anything in certain cases.
#[cfg(not(feature = "no_std"))]
    fn write_to(&self, io : &mut Write) -> io::Result<()> {
        try!(write!(io, "{}", self.as_str()));
        Ok(())
    }

}

// TODO: why does this conflict with &'a str?
// impl<T : AsRef<str>> Into<Cow<'a, str>> for T {
// fn as_str(&self) -> Cow<str> {
// Cow::Borrowed(self.as_ref())
// }
// }
//

impl<'a> Message for &'a str {
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self)
    }
}

impl Message for String {
    fn as_str(&self) -> Cow<str> {
        Cow::Borrowed(self.as_str())
    }
}

impl<'a> Message for fmt::Arguments<'a> {
    fn as_str(&self) -> Cow<str> {
        let mut s = String::new();
        fmt::write(&mut s, *self).unwrap();
        Cow::Owned(s)
    }

#[cfg(not(feature = "no_std"))]
    fn write_to(&self, io : &mut Write) -> io::Result<()> {
        try!(write!(io, "{}", self));
        Ok(())
    }
}

impl Logger {
    /// Build a root logger
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    ///
    /// Use `o!` macro to help build `values`
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(
    ///         slog::discard(),
    ///         o!("key1" => "value1", "key2" => "value2"),
    ///     );
    /// }
    pub fn root<D: 'static + Drain<Error=()> + Sized>(d: D, values: Vec<OwnedKeyValue>) -> Logger {
        Logger {
            drain: Arc::new(d),
            values: Arc::new(OwnedKeyValueList::root(values)),
        }
    }

    /// Build a child logger
    ///
    /// Child logger copies all existing values from the parent.
    ///
    /// All children, their children and so on, form one hierarchy sharing
    /// a common drain.
    ///
    /// Use `o!` macro to help build `values`
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(slog::discard(),
    ///         o!("key1" => "value1", "key2" => "value2"));
    ///     let log = root.new(o!("key" => "value"));
    /// }
    pub fn new(&self, values: Vec<OwnedKeyValue>) -> Logger {
        Logger {
            drain: self.drain.clone(),
            values: Arc::new(OwnedKeyValueList::new(values, self.values.clone())),
        }
    }

    /// Log one logging record
    ///
    /// Use specific logging functions instead.
    #[inline]
    pub fn log(&self, record: &Record) {
        let _ = self.drain.log(&record, &*self.values);
    }
}

/// Logging record
pub struct Record<'a> {
    /// Logging level
    level: Level,
    /// Message
    msg: &'a Message,
    /// File
    file: &'static str,
    /// Line
    line: u32,
    /// Module
    module: &'static str,
    /// Target - for backward compatibility with `log`
    target: &'a str,
    /// Values
    values: &'a [BorrowedKeyValue<'a>],
}


impl<'a> Record<'a> {
    /// Create a new `Record`
    #[inline]
    pub fn new(
               level: Level,
               msg: &'a Message,
               file: &'static str,
               line: u32,
               module: &'static str,
               target: &'a str,
               values: &'a [BorrowedKeyValue<'a>])
               -> Self {
        Record {
            level: level,
            msg: msg,
            file: file,
            line: line,
            module: module,
            target: target,
            values: values,
        }
    }

    /// Get a log record message
    pub fn msg(&self) -> Cow<str> {
        self.msg.as_str()
    }

    /// Get record logging level
    pub fn level(&self) -> Level {
        self.level
    }

    /// Get line number
    pub fn line(&self) -> u32 {
        self.line
    }

    /// Get file path
    pub fn file(&self) -> &'static str {
        self.file
    }

    /// Get target
    ///
    /// Mostly for backward compatibility with `log`
    pub fn target(&self) -> &str {
        self.target
    }

    /// Get module
    pub fn module(&self) -> &'static str {
        self.module
    }

    /// Record value-key pairs
    pub fn values(&self) -> &'a [BorrowedKeyValue<'a>] {
        self.values
    }
}
