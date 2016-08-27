use std::cell::RefCell;
use std::borrow::Cow;
use std::io::Write;

thread_local! {
    static TL_BUF: RefCell<Vec<u8>> = RefCell::new(Vec::with_capacity(128))
}

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
    drain: Arc<Drain>,
    values: Arc<OwnedKeyValueList>,
}

/// Log Record message
pub trait Message {
    /// Take a string
    fn as_str(&self) -> Cow<str>;

    /// Write it into `io` (which might be fast than `as_str()` it does not
    /// need to allocate anything in certain cases.
    fn write_to(&self, io : &mut Write) -> Result<()> {
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

    fn write_to(&self, io : &mut Write) -> Result<()> {
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
    pub fn root<D: 'static + Drain + Sized>(d: D, values: Vec<OwnedKeyValue>) -> Logger {
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
    pub fn log(&self, record: &Record) {

        // By default errors in loggers are ignored
        TL_BUF.with(|buf| {
            let mut buf = buf.borrow_mut();
            let _ = self.drain.log(&mut *buf, &record, &*self.values);
            // TODO: Double check if this will not zero the old bytes as it costs time
            debug_assert!(buf.is_empty());
        });
    }
}

/// Logging record
pub struct Record<'a> {
    /// Lazily initialized timestamp
    ///
    /// Since getting current time requires a
    /// syscall and most log records won't make it to the final drain it will be
    /// delayed until ts is actually needed.
    ts: RefCell<Option<chrono::DateTime<chrono::UTC>>>,
    /// Logging level
    level: Level,
    /// Message
    msg: &'a Message,
    /// File
    file: &'a str,
    /// Line
    line: u32,
    /// Module
    module: &'a str,
    /// Values
    values: &'a [BorrowedKeyValue<'a>],
}

impl<'a> Record<'a> {
    /// Create a new `Record`
    #[inline]
    pub fn new(level: Level,
               msg: &'a Message,
               file: &'a str,
               line: u32,
               module: &'a str,
               values: &'a [BorrowedKeyValue<'a>])
               -> Self {
        Record {
            ts: RefCell::new(None),
            level: level,
            msg: msg,
            file: file,
            line: line,
            module: module,
            values: values,
        }
    }

    /// Timestamp
    ///
    /// Lazily evaluated timestamp
    pub fn ts(&self) -> chrono::DateTime<chrono::UTC> {
        let mut ts = self.ts.borrow_mut();
        match *ts {
            None => {
                let now = chrono::UTC::now();
                *ts = Some(now);
                now
            }
            Some(ts) => ts,
        }
    }


    /// Set timestamp
    ///
    /// Useful for manually forcing timestamp value (eg. in testing).
    pub fn set_ts(&self, ts: chrono::DateTime<chrono::UTC>) {
        *self.ts.borrow_mut() = Some(ts);
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
    pub fn file(&self) -> &str {
        self.file

    }

    /// Get module
    pub fn module(&self) -> &str {
        self.module
    }

    /// Record value-key pairs
    pub fn values(&self) -> &'a [BorrowedKeyValue<'a>] {
        self.values
    }
}
