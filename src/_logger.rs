

/// Logging handle used to execute logging statements
///
/// Logger handles logging context (key-value list) and handles logging
/// statements.
///
/// Child loggers are build from existing loggers, and inherit existing
/// key-value pairs from their parents, which can be supplemented with
/// new ones.
///
/// Cloning existing loggers and creating new ones is cheap. Loggers can be
/// freely passed around the code.
#[derive(Clone)]
pub struct Logger {
    drain: Arc<Drain<Error=Never>+Send+Sync>,
    list: OwnedKVList,
}

impl Logger {
    /// Build a root `Logger`
    ///
    /// All children and their children and so on form one hierarchy
    /// sharing a common drain.
    ///
    /// Root logger starts a new hierarchy associated with a given `Drain`. Root
    /// logger drain must return no errors. See `DrainExt::ignore_err()` and
    ///
    /// `DrainExt::fuse()`.
    /// Use `o!` macro to help build key-value pairs with a nicer syntax.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let _root = slog::Logger::root(
    ///         slog::Discard,
    ///         o!("key1" => "value1", "key2" => "value2"),
    ///     );
    /// }
    pub fn root<D>(d: D, values: OwnedKVGroup) -> Logger
    where D: 'static + Drain<Error=Never> + Sized+Send+Sync{
        Logger {
            drain: Arc::new(d),
            list: OwnedKVList::root(values),
        }
    }

    /// Build a child logger
    ///
    /// Child logger inherits all existing key-value pairs from it's parent.
    ///
    /// All children, their children and so on, form one hierarchy sharing
    /// a common drain.
    ///
    /// Use `o!` macro to help build key value pairs using nicer syntax.
    ///
    /// ```
    /// #[macro_use]
    /// extern crate slog;
    ///
    /// fn main() {
    ///     let root = slog::Logger::root(slog::Discard,
    ///         o!("key1" => "value1", "key2" => "value2"));
    ///     let _log = root.new(o!("key" => "value"));
    /// }
    pub fn new(&self, values: OwnedKVGroup) -> Logger {
        Logger {
            drain: self.drain.clone(),
            list: OwnedKVList::new(values, &self.list),
        }
    }

    /// Log one logging record
    ///
    /// Use specific logging functions instead. See `log!` macro
    /// documentation.
    #[inline]
    pub fn log(&self, record: &Record) {
        let _ = self.drain.log(
            &record,
            &self.list,
            );
    }
}

impl fmt::Debug for Logger {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        try!(write!(f, "Logger{:?}", self.list));
        Ok(())
    }
}

impl Drain for Logger {

    type Error = Never;

    fn log(&self, record: &Record, values : &OwnedKVList) -> result::Result<(), Self::Error> {
        debug_assert!(self.list.next_list.is_none());

        let chained = values.append(&self.list);
        self.drain.log(
            &record,
            &chained,
            )
    }
}

