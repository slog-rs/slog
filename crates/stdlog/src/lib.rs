//! Standard Rust log crate adapter to slog-rs
//!
//! This crate allows setting one of `slog` `Logger`s
//! as a global handler of Rust de facto standard logging statements
//! (eg ``info!(...)``) provided by `log` create.
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! #[macro_use]
//! extern crate log;
//! extern crate slog_stdlog;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!     slog_stdlog::set_logger(root).unwrap();
//!     info!("standard logging redirected to slog");
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate log;

use log::LogMetadata;
use std::sync;

use slog::Level;

// TODO: Change this to use thread local copies
struct Logger(sync::Mutex<slog::Logger>);

impl Logger {
    fn new(logger: slog::Logger) -> Self {
        Logger ( sync::Mutex::new(logger) )
    }
}

fn log_to_slog_level(level: log::LogLevel) -> Level {
    match level {
        log::LogLevel::Trace => Level::Trace,
        log::LogLevel::Debug => Level::Debug,
        log::LogLevel::Info => Level::Info,
        log::LogLevel::Warn => Level::Warning,
        log::LogLevel::Error => Level::Error,
    }
}


impl log::Log for Logger {
    fn enabled(&self, _: &LogMetadata) -> bool {
        true
    }

    fn log(&self, r: &log::LogRecord) {
        let level = log_to_slog_level(r.metadata().level());

        let msg = r.args();
        let target = r.target();
        let module = r.location().module_path();
        let file = r.location().file();
        let line = r.location().line();
        {
            let _ = self.0.lock()
                .map(|l| (*l).log(level,
                               msg,
                               &[("target", &target),
                                 ("module", &module),
                                 ("file", &file),
                                 ("line", &line)]));
        }
    }
}

/// Set a `slog::Logger` as a global `log` create handler
///
/// This will forward all `log` records to `slog` logger.
pub fn set_logger(logger: slog::Logger) -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::max());
        Box::new(Logger::new(logger))
    })
}

/// Set a `slog::Logger` as a global `log` create handler
///
/// This will forward `log` records that satisfy `log_level_filter` to `slog` logger.
pub fn set_logger_level(logger: slog::Logger,
                        log_level_filter: log::LogLevelFilter)
                        -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log_level_filter);
        Box::new(Logger::new(logger))
    })
}
