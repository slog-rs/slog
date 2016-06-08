#[derive(Copy, Clone, Debug)]
pub enum Level {
    Critical,
    Error,
    Warning,
    Info,
    Debug,
    Trace
}

impl Level {
    pub fn as_str(&self) -> &'static str {
        match *self {
            Level::Critical => "CRIT",
            Level::Error => "ERRO",
            Level::Warning => "WARN",
            Level::Info => "INFO",
            Level::Debug => "DEBG",
            Level::Trace => "TRCE",
        }
    }

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
        write!(f, "{}", self.as_str())
    }
}

impl Level {
    fn is_at_least(&self, level : Self) -> bool {
        self.as_int() <= level.as_int()
    }
}

#[test]
fn level_at_least() {
    assert!(Level::Debug.is_at_least(Level::Debug));
    assert!(Level::Debug.is_at_least(Level::Trace));
    assert!(!Level::Debug.is_at_least(Level::Info));
}

