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
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

use log::LogMetadata;
use std::sync::Arc;
use std::cell::RefCell;
use std::io;

use slog::Level;
use crossbeam::sync::ArcCell;

thread_local! {
    static TL_SCOPES: RefCell<Vec<slog::Logger>> = RefCell::new(Vec::with_capacity(8))
}

lazy_static! {
    static ref GLOBAL_LOGGER : ArcCell<slog::Logger> = ArcCell::new(
        Arc::new(
            slog::Logger::root(slog::discard(), o!())
        )
    );
}

fn set_global_logger(l : slog::Logger) {
    let _ = GLOBAL_LOGGER.set(Arc::new(l));
}

struct Logger;


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
        let target = r.target();
        let module = r.location().__module_path;
        let file = r.location().__file;
        let line = r.location().line();
        with_current_logger(
            |l| l.log(&slog::Record::new(level, args, file, line, module, target, &[]))
        )
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
/// fn main() {
///     let root = slog::Logger::root(
///         slog::discard(),
///         o!("build-id" => "8dfljdf"),
///     );
///     slog_stdlog::set_logger(root).unwrap();
///     // Note: this `info!(...)` macro comes from `log` crate
///     info!("standard logging redirected to slog");
/// }
/// ```
pub fn set_logger(logger: slog::Logger) -> Result<(), log::SetLoggerError> {
    log::set_logger(|max_log_level| {
        max_log_level.set(log::LogLevelFilter::max());
        set_global_logger(logger);
        Box::new(Logger)
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
        set_global_logger(logger);
        Box::new(Logger)
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
    let drain = slog::level_filter(Level::Info, slog_term::streamer().compact().build());
    set_logger(slog::Logger::root(drain, o!()))
}

struct ScopeGuard;


impl ScopeGuard {
    fn new(logger : slog::Logger) -> Self {
        TL_SCOPES.with(|s| {
            s.borrow_mut().push(logger);
        });

        ScopeGuard
    }
}

impl Drop for ScopeGuard {
    fn drop(&mut self) {
        TL_SCOPES.with(|s| {
            s.borrow_mut().pop().expect("TL_SCOPES should contain a logger");
        })
    }
}


/// Access the currently active logger
///
/// The reference logger will be either:
/// * global logger, or
/// * currently active scope logger
///
/// **Warning**: Calling `scope` inside `f`
/// will result in a panic.
pub fn with_current_logger<F, R>(f : F) -> R
                           where F : FnOnce(&slog::Logger) -> R {
    TL_SCOPES.with(|s| {
        let s = s.borrow();
        if s.is_empty() {
            f(&GLOBAL_LOGGER.get())
        } else {
            f(&s[s.len() - 1])
        }
    })
}

/// Execute code in a logging scope
///
/// Logging scopes allow using different logger for legacy logging
/// statements in part of the code.
///
/// Logging scopes can be nested and are panic safe.
///
/// `logger` is the `Logger` to use during the duration of `f`.
/// `with_current_logger` can be used to build it as a child of currently active
/// logger.
///
/// `f` is a code to be executed in the logging scope.
///
/// Note: Thread scopes are thread-local. Each newly spawned thread starts
/// with a global logger, as a current logger.
pub fn scope<SF, R>(logger : slog::Logger, f : SF) -> R
    where SF : FnOnce() -> R
{
    let _guard = ScopeGuard::new(logger);
    f()
}

/// Drain logging `Record`s into `log` crate
///
/// Using `StdLog` is effectively the same as using `log::info!(...)` and
/// other standard logging statements.
///
/// Caution needs to be taken to prevent circular loop where `Logger`
/// installed via `slog-stdlog::set_logger` would log things to a `StdLog`
/// drain, which would again log things to the global `Logger` and so on
/// leading to an infinite recursion.
pub struct StdLog;

impl slog::Drain for StdLog {
    fn log(&self, info: &slog::Record, _ : &slog::OwnedKeyValueList) -> io::Result<()> {

        let level = match info.level() {
            slog::Level::Critical => log::LogLevel::Error,
            slog::Level::Error => log::LogLevel::Error,
            slog::Level::Warning => log::LogLevel::Warn,
            slog::Level::Info => log::LogLevel::Info,
            slog::Level::Debug => log::LogLevel::Debug,
            slog::Level::Trace => log::LogLevel::Trace,
        };

        let target = info.target();

        let location = log::LogLocation {
            __module_path: info.module(),
            __file: info.file(),
            __line: info.line(),
        };

        // Please don't yell at me for this! :D
        // https://github.com/rust-lang-nursery/log/issues/95
        log::__log(level, target, &location, format_args!("{}", info.msg()));

        Ok(())
    }
}
