/// Logging drain
///
/// Drains generally mean destination for logs, but slog generalize the
/// term. `Drain`-s are responsible for filtering, formatting and writing the
/// log records into given destination.
///
/// Implementing this trait allows writing own Drains, that can be combined
/// with other drains.
pub trait Drain: Send + Sync {
    /// Type of potential error returned during logging
    type Error;
    /// Write one logging record
    /// As an optimization (avoiding allocations), loggers are responsible for
    /// providing a byte buffer, that `Drain` can use for their own needs.
    fn log(&self, info: &Record, &OwnedKeyValueList) -> result::Result<(), Self::Error>;
}

impl<D: Drain+?Sized> Drain for Box<D> {
    type Error = D::Error;
    fn log(&self, info: &Record, o: &OwnedKeyValueList) -> result::Result<(), D::Error> {
        (**self).log(info, o)
    }
}

impl<D: Drain+?Sized> Drain for Arc<D> {
    type Error = D::Error;
    fn log(&self, info: &Record, o: &OwnedKeyValueList) -> result::Result<(), D::Error> {
        (**self).log(info, o)
    }
}

/// Fusing trait
///
/// This trait is implemented for all the `Drain`s,
/// allowing convenient error handling.
pub trait Fuse : Sized + Drain {
    /// Make `Self` panic when returning any errors
    fn fused(self) -> PanicFuse<Self> {
        panic_fuse(self)

    }

    /// Make `Self` ignore and not report any error
    fn unfused(self) -> IgnoreFuse<Self> {
        ignore_fuse(self)
    }
}

impl<D : Drain> Fuse for D where D::Error : core::fmt::Display{}

/// Drain discarding everything
pub struct Discard;

impl Drain for Discard {
    type Error = ();
    fn log(&self, _: &Record, _: &OwnedKeyValueList) -> result::Result<(), ()> {
        Ok(())
    }
}

/// Drain filtering records
///
/// Wraps a `Drain` and passes `Record`-s to it, only if they satisifies a
/// condition `cond`.
pub struct Filter<D: Drain> {
    drain: D,
    // eliminated dynamic dispatch, after rust learns `-> impl Trait`
    cond: Box<Fn(&Record) -> bool + 'static + Send + Sync>,
}

impl<D: Drain> Filter<D> {
    /// Create Filter wrapping given `subdrain` and passing to it records
    /// only the `cond` is true
    pub fn new<F: 'static + Sync + Send + Fn(&Record) -> bool>(drain: D, cond: F) -> Self {
        Filter {
            drain: drain,
            cond: Box::new(cond),
        }
    }
}

impl<D: Drain> Drain for Filter<D> {
    type Error = D::Error;
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> result::Result<(), Self::Error> {
        if (self.cond)(&info) {
            self.drain.log(info, logger_values)
        } else {
            Ok(())
        }
    }
}

/// Record log level filter
///
/// Wraps a drain and passes records to it, only
/// if their level is at least given level.
///
/// TODO: Remove this type. This drain is a special case of `Filter`, but
/// because `Filter` can not use static dispatch ATM due to Rust limitations
/// that will be lifted in the future, it is a standalone type.
pub struct LevelFilter<D: Drain> {
    level: Level,
    drain: D,
}

impl<D: Drain> LevelFilter<D> {
    /// Create `LevelFilter`
    pub fn new(drain: D, level: Level) -> Self {
        LevelFilter {
            level: level,
            drain: drain,
        }
    }
}

impl<D: Drain> Drain for LevelFilter<D> {
    type Error = D::Error;
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> result::Result<(), Self::Error> {
        if info.level().is_at_least(self.level) {
            self.drain.log(info, logger_values)
        } else {
            Ok(())
        }
    }
}

/// Drain duplicating records to two sub-drains
///
/// Can be nested for more than two outputs.
pub struct Duplicate<D1: Drain, D2: Drain> {
    drain1: D1,
    drain2: D2,
}


impl<D1: Drain, D2: Drain> Duplicate<D1, D2> {
    /// Create `Duplicate`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Duplicate {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1, D2, E> Drain for Duplicate<D1, D2>
where
D1 : Drain<Error = E>,
D2 : Drain<Error = E>
{
    type Error = D1::Error;
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> result::Result<(), Self::Error> {
        let res1 = self.drain1.log(info, logger_values);
        let res2 = self.drain2.log(info, logger_values);

        // TODO: Don't discard e2 in case of two errors at once?
        match (res1, res2) {
            (Ok(_), Ok(_)) => Ok(()),
            (Ok(_), Err(e)) => Err(e),
            (Err(e), Ok(_)) => Err(e),
            (Err(e1), Err(_)) => Err(e1),
        }
    }
}

/// Failover drain
///
/// Log everything to logger `D1`, but in case of it reporting error,
/// try logging to `D2`. If it also returned an error, forward it up.
pub struct Failover<D1: Drain, D2: Drain> {
    drain1: D1,
    drain2: D2,
}

impl<D1: Drain, D2: Drain> Failover<D1, D2> {
    /// Create `Failover`
    pub fn new(drain1: D1, drain2: D2) -> Self {
        Failover {
            drain1: drain1,
            drain2: drain2,
        }
    }
}

impl<D1, D2, E1, E2> Drain for Failover<D1, D2>
where
D1 : Drain<Error = E1>,
D2 : Drain<Error = E2>
{
    type Error = D2::Error;
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
           -> result::Result<(), Self::Error> {
        match self.drain1.log(info, logger_values) {
            Ok(_) => Ok(()),
            Err(_) => self.drain2.log(info, logger_values),
        }
    }
}

/// Panicking fuse
///
/// `Logger` requires a root drain to handle all errors (`Drain::Error == ()`),
/// `PanicFuse` will wrap a `Drain` and panic if it returns any errors.
///
/// Note: `Drain::Error` must implement `Display`. It's easy to create own
/// `Fuse` drain if this requirement can't be fulfilled.
pub struct PanicFuse<D: Drain> {
    drain: D,
}

impl<D: Drain> PanicFuse<D> {
    /// Create PanicFuse wrapping given `subdrain`
    pub fn new(drain: D) -> Self {
        PanicFuse {
            drain: drain,
        }
    }
}

impl<D: Drain> Drain for PanicFuse<D> where D::Error : core::fmt::Display {
    type Error = ();
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
        -> result::Result<(), ()> {
            Ok(
                self.drain.log(info, logger_values).unwrap_or_else(
                    |e| panic!("PanicFuse: {}", e)
                    )
                )
        }
}


/// Error ignoring fuse
///
/// `Logger` requires a root drain to handle all errors (`Drain::Error == ()`),
/// `IgnoreFuse` will ignore all errors of the drain it wraps.
pub struct IgnoreFuse<D: Drain> {
    drain: D,
}

impl<D: Drain> IgnoreFuse<D> {
    /// Create `IgnoreFuse` wrapping given `subdrain`
    pub fn new(drain: D) -> Self {
        IgnoreFuse {
            drain: drain,
        }
    }
}

impl<D: Drain> Drain for IgnoreFuse<D> {
    type Error = ();
    fn log(&self,
           info: &Record,
           logger_values: &OwnedKeyValueList)
        -> result::Result<(), ()> {
            let _ = self.drain.log(info, logger_values);
            Ok(())
        }
}

/// Discard all logging records
///
/// Create a Discard drain
pub fn discard() -> Discard {
    Discard
}

/// Filter by `cond` closure
pub fn filter<D: Drain, F: 'static + Send + Sync + Fn(&Record) -> bool>(
    cond: F,
    d: D
    ) -> Filter<D> {
    Filter::new(d, cond)
}

/// Filter by log level
pub fn level_filter<D: Drain>(level: Level, d: D) -> LevelFilter<D> {
    LevelFilter::new(d, level)
}

/// Duplicate records to two drains
///
/// Create `Duplicate` drain.
///
/// Can be nested for multiple outputs.
pub fn duplicate<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Duplicate<D1, D2> {
    Duplicate::new(d1, d2)
}

/// Failover logging to secondary drain on primary's failure
///
/// Create `Failover` drain
pub fn failover<D1: Drain, D2: Drain>(d1: D1, d2: D2) -> Failover<D1, D2> {
    Failover::new(d1, d2)
}

/// Panic if the subdrain returns an error.
///
/// Create `PanicFuse` drain
pub fn panic_fuse<D: Drain>(d: D) -> PanicFuse<D> {
    PanicFuse::new(d)
}

/// Ignore any errors returned by the subdrain
///
/// Create `IgnoreFuse` drain
pub fn ignore_fuse<D: Drain>(d: D) -> IgnoreFuse<D> {
    IgnoreFuse::new(d)
}
