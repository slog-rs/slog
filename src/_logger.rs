

/// Logging handle used to execute logging statements
///
/// `Logger` holds logging context (key-value pairs) and handles logging
/// statements by delivering all logging statement information (`Record`)
/// to it's `Drain`.
///
/// Child loggers are build from existing loggers, and inherit their existing
/// key-value pairs, which can be supplemented with new ones.
///
/// Cloning existing loggers and creating new ones is cheap. Loggers can be
/// freely passed around the code and between threads.
///
/// `Logger`s are also `Sync` - there's no need to synchronize accesses to them,
/// and they can accept logging records from multiple threads at once. Because
/// of that they require the `Drain` to be `Sync+Sync` as well. Not all `Drain`s
/// are `Sync` or `Send` but they can often be made so by wrapping in a `Mutex`
/// and/or `Arc`.
#[derive(Clone)]
pub struct Logger {
    drain: Arc<Drain<Err=Never,Ok=()>+Send+Sync>,
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
    /// `DrainExt::fuse()`.
    ///
    /// Use `o!` macro to build `OwnedKV` object..
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
    pub fn root<D>(d: D, values: OwnedKV) -> Logger
    where D: 'static + Drain<Err=Never, Ok=()> + Sized+Send+Sync{
        Logger {
            drain: Arc::new(d),
            list: OwnedKVList::root(values),
        }
    }

    /// Build a child logger
    ///
    /// Child logger inherits all existing key-value pairs from it's parent.
    ///
    /// All children, their children and so on, form one hierarchy sharing a
    /// common drain.
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
    pub fn new(&self, values: OwnedKV) -> Logger {
        Logger {
            drain: self.drain.clone(),
            list: OwnedKVList::new(values, self.list.node.clone()),
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

    type Ok = ();
    type Err = Never;

    fn log(&self, record: &Record, values : &OwnedKVList) -> result::Result<Self::Ok, Self::Err> {
        debug_assert!(self.list.next_list.is_none());

        let chained = values.append(&self.list);
        self.drain.log(
            &record,
            &chained,
            )
    }
}

