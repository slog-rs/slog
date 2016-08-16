/// Logging level
#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
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


static LOG_LEVEL_NAMES: [&'static str; 7] = ["OFF", "CRITICAL", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[repr(usize)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
#[doc(hidden)]
/// Not part of the API
pub enum FilterLevel {
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

    /// Get a `Level` from a usize
    ///
    /// This complements `as_usize`
    pub fn from_usize(u: usize) -> Option<Level> {
        match u {
            1 => Some(Level::Critical),
            2 => Some(Level::Error),
            3 => Some(Level::Warning),
            4 => Some(Level::Info),
            5 => Some(Level::Debug),
            6 => Some(Level::Trace),
            _ => None
        }
    }
}

impl FilterLevel {
    #[doc(hidden)]
    /// Not part of the API
    pub fn as_usize(&self) -> usize {
        match *self {
            FilterLevel::Off => 0,
            FilterLevel::Critical => 1,
            FilterLevel::Error => 2,
            FilterLevel::Warning => 3,
            FilterLevel::Info => 4,
            FilterLevel::Debug => 5,
            FilterLevel::Trace => 6,
        }
    }

    /// Get a `FilterLevel` from a usize
    ///
    /// This complements `as_usize`
    pub fn from_usize(u: usize) -> Option<FilterLevel> {
        match u {
            0 => Some(FilterLevel::Off),
            1 => Some(FilterLevel::Critical),
            2 => Some(FilterLevel::Error),
            3 => Some(FilterLevel::Warning),
            4 => Some(FilterLevel::Info),
            5 => Some(FilterLevel::Debug),
            6 => Some(FilterLevel::Trace),
            _ => None
        }
    }

    /// Maximum logging level (log everything)
    pub fn max() -> Self {
        FilterLevel::Trace
    }
}

impl FromStr for FilterLevel {
    type Err = ();
    fn from_str(level: &str) -> std::result::Result<FilterLevel, ()> {
        LOG_LEVEL_NAMES.iter()
            .position(|&name| name.eq_ignore_ascii_case(level))
            .map(|p| FilterLevel::from_usize(p).unwrap()).ok_or(())
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
fn filterlevel_sanity() {
    assert!(Level::Critical.as_usize() > FilterLevel::Off.as_usize());
    assert!(Level::Critical.as_usize() <= FilterLevel::Critical.as_usize());
    assert!(Level::Trace.as_usize() <= FilterLevel::Trace.as_usize());
}
