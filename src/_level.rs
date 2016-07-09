/// Logging level
#[derive(Copy, Clone, Debug)]
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

    /// Cast `Level` to ordering integer, where
    /// `Critical` is 0, and `Trace` is 5
    pub fn as_int(&self) -> i32 {
        match *self {
            Level::Critical => 0,
            Level::Error => 1,
            Level::Warning => 2,
            Level::Info => 3,
            Level::Debug => 4,
            Level::Trace => 5,
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
        self.as_int() <= level.as_int()
    }
}

#[test]
fn level_at_least() {
    assert!(Level::Debug.is_at_least(Level::Debug));
    assert!(Level::Debug.is_at_least(Level::Trace));
    assert!(!Level::Debug.is_at_least(Level::Info));
}

