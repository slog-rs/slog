/// Logging level
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Level {
    /// Critical
    Critical,
    /// Error
    Error,
    /// Warning
    Warning,
    /// Info
    Info,
    /// Debug
    Debug,
    /// Trace
    Trace
}

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
#[doc(hidden)]
/// Not part of the API
pub enum LevelFilter {
    Off,
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace,
}

impl Level {
    /// Convert to short string
    ///
    /// Currently a 4 char string.
    pub fn as_short_str(&self) -> &'static str {
        match *self {
            Level::Critical => "CRIT",
            Level::Error => "ERRO",
            Level::Warning => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBG",
            Level::Trace => "TRCE",
        }
    }

    /// Cast `Level` to ordering integer where `Critical` is the smallest and
    /// `Trace` the biggest value
    pub fn as_usize(&self) -> usize {
        match *self {
            Level::Critical => 1,
            Level::Error => 2,
            Level::Warning => 3,
            Level::Info => 4,
            Level::Debug => 5,
            Level::Trace => 6,
        }
    }
}

impl LevelFilter {
    #[doc(hidden)]
    /// Not part of the API
    pub fn as_usize(&self) -> usize {
        match *self {
            LevelFilter::Off => 0,
            LevelFilter::Critical => 1,
            LevelFilter::Error => 2,
            LevelFilter::Warning => 3,
            LevelFilter::Info => 4,
            LevelFilter::Debug => 5,
            LevelFilter::Trace => 6,
        }
    }
}

impl fmt::Display for Level {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_short_str())
    }
}

impl Level {
    /// Returns true if `self` is at least `level` logging level
    pub fn is_at_least(&self, level : Self) -> bool {
        self.as_usize() <= level.as_usize()
    }
}

#[test]
fn level_at_least() {
    assert!(Level::Debug.is_at_least(Level::Debug));
    assert!(Level::Debug.is_at_least(Level::Trace));
    assert!(!Level::Debug.is_at_least(Level::Info));
}

#[test]
fn levelfilter_sanity() {
    assert!(Level::Critical.as_usize() > LevelFilter::Off.as_usize());
    assert!(Level::Critical.as_usize() <= LevelFilter::Critical.as_usize());
    assert!(Level::Trace.as_usize() <= LevelFilter::Trace.as_usize());
}
