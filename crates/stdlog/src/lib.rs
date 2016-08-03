//! Standard Rust log crate adapter to slog-rs
//!
//! This crate allows using `slog` features with code
//! using legacy `log` statements.
//!
//! `log` crate expects a global logger to be registered
//! (popular one is `env_logger`) as a handler for all
//! `info!(...)` and similar.
//!
//! `slog-stdlog` will register itself as `log` global handler and forward all
//! legacy logging statements to `slog`'s `Logger`. That means existing logging
//! `debug!` (even in dependencies crates) work and utilize `slog` composable
//! drains.
//!
//! See `init()` documentation for minimal working example.
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate log;

use log::LogMetadata;
use std::sync;

use slog::Level;

// TODO: Change this to use thread local copies
struct Logger(sync::Mutex<slog::Logger>);

impl Logger {
    fn new(logger: slog::Logger) -> Self {
        Logger(sync::Mutex::new(logger))
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

        let args = r.args();
        // TODO: What to do with it?
        let _target = r.target();
        let module = r.location().module_path();
        let file = r.location().file();
        let line = r.location().line();
        {
            let _ = self.0
                .lock()
                .map(|l| (*l).log(&slog::RecordInfo::new(level, args, file, line, module, &[])));
        }
    }
}

/// Set a `slog::Logger` as a global `log` create handler
///
/// This will forward all `log` records to `slog` logger.
///
/// ```
/// // only use `o` macro from `slog` crate
/// #[macro_use(o)]
/// extern crate slog;
/// #[macro_use]
/// extern crate log;
/// extern crate slog_stdlog;
///
/// use slog::*;
///
/// fn main() {
///     let root = Logger::new_root(
///         o!("build-id" => "8dfljdf"),
///         slog::drain::discard(),
///     );
///     slog_stdlog::set_logger(root).unwrap();
///     // Note: this `info!(...)` macro comes from `log` crate
///     info!("standard logging redirected to slog");
/// }
/// ```
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

/// Minimal initialization with default drain
///
/// The exact default drain is unspecified and will
/// change in future versions! Use `set_logger` instead
/// to build customized drain.
///
/// ```
/// #[macro_use]
/// extern crate log;
/// extern crate slog_stdlog;
///
/// fn main() {
///     slog_stdlog::init().unwrap();
///     // Note: this `info!(...)` macro comes from `log` crate
///     info!("standard logging redirected to slog");
/// }
/// ```
pub fn init() -> Result<(), log::SetLoggerError> {
    use slog::drain::IntoLogger;
    let drain = slog::drain::filter_level(Level::Info, slog_term::stderr());
    set_logger(drain.into_logger(o!()))
}
