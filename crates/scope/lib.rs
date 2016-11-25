//! Logging scopes for slog-rs
//!
//! Logging scopes are convinience functionality for slog-rs to free user from manually passing
//! `Logger` objects around.
//!
//! Set of macros is also provided as an alternative to original `slog` crate macros, for logging
//! directly to `Logger` of the current logging scope.
//!
//! Note: Part of a `slog` logging philosophy is ability to freely express logging contexts
//! according to logical structure, rather than code structure. By using logging scopes the logging
//! context is tied to code flow again, which is less expressive.
//!
//! ```
//! #[macro_use(o, slog_info, slog_log)]
//! extern crate slog;
//! #[macro_use]
//! extern crate slog_scope;
//! extern crate slog_term;
//!
//! use slog::DrainExt;
//!
//! fn foo() {
//!     slog_info!(slog_scope::logger(), "foo");
//!     info!("foo"); // Same as above, but more ergonomic.
//! }
//!
//! fn main() {
//!     let log = slog::Logger::root(slog_term::streamer().stderr().build().fuse(), o!("version" => "0.5"));
//!
//!     slog_scope::set_global_logger(log);
//!     slog_scope::scope(slog_scope::logger().new(o!("scope" => "1")),
//!         || foo()
//!     );
//! }


#![warn(missing_docs)]

#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

use slog::*;

use std::sync::Arc;
use std::cell::RefCell;
use crossbeam::sync::ArcCell;

/// Log a critical level message using current scope logger
#[macro_export] macro_rules! crit( ($($args:tt)+) => { slog_crit![$crate::logger(), $($args)+]; };);
/// Log a error level message using current scope logger
#[macro_export] macro_rules! error( ($($args:tt)+) => { slog_error![$crate::logger(), $($args)+]; };);
/// Log a warning level message using current scope logger
#[macro_export] macro_rules! warn( ($($args:tt)+) => { slog_warn![$crate::logger(), $($args)+]; };);
/// Log a info level message using current scope logger
#[macro_export] macro_rules! info( ($($args:tt)+) => { slog_info![$crate::logger(), $($args)+]; };);
/// Log a debug level message using current scope logger
#[macro_export] macro_rules! debug( ($($args:tt)+) => { slog_debug![$crate::logger(), $($args)+]; };);
/// Log a trace level message using current scope logger
#[macro_export] macro_rules! trace( ($($args:tt)+) => { slog_trace![$crate::logger(), $($args)+]; };);

thread_local! {
    static TL_SCOPES: RefCell<Vec<slog::Logger>> = RefCell::new(Vec::with_capacity(8))
}

lazy_static! {
    static ref GLOBAL_LOGGER : ArcCell<slog::Logger> = ArcCell::new(
        Arc::new(
            slog::Logger::root(slog::Discard, o!())
        )
    );
}

/// Set global `Logger` that is returned by calls like `logger()` outside of any logging scope.
pub fn set_global_logger(l: slog::Logger) {
    let _ = GLOBAL_LOGGER.set(Arc::new(l));
}

struct ScopeGuard;


impl ScopeGuard {
    fn new(logger: slog::Logger) -> Self {
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

/// Access the `Logger` for the current logging scope
pub fn logger() -> Logger {
    TL_SCOPES.with(|s| {
        let s = s.borrow();
        match s.last() {
          Some(logger) => logger.clone(),
          None => (*GLOBAL_LOGGER.get()).clone(),
        }
    })
}

/// Execute code in a logging scope
///
/// Logging scopes allow using a `slog::Logger` without explicitly
/// passing it in the code.
///
/// At any time current active `Logger` for a given thread can be retrived
/// with `logger()` call.
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
pub fn scope<SF, R>(logger: slog::Logger, f: SF) -> R
    where SF: FnOnce() -> R
{
    let _guard = ScopeGuard::new(logger);
    f()
}
