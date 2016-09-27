//! Logging scopes for slog-rs
//!
//! Logging scopes are convinience functionality for slog-rs that free user from manually passing
//! `Logger` objects around
//!
//! Note: Part of a `slog` logging philosophy is ability to freelly express logging contexts
//! acording to logical structure, rather than code structure. By using logging scopes logging
//! context is tied to code flow again, which is less expressive.
//!
//! ```
//!#[macro_use]
//! extern crate slog;
//! extern crate slog_scope;
//! extern crate slog_term;
//!
//! use slog::DrainExt;
//!
//! fn foo() {
//!     info!(slog_scope::logger(), "foo");
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

#[macro_use]
extern crate slog;
#[macro_use]
extern crate lazy_static;
extern crate crossbeam;

use slog::*;

use std::sync::Arc;
use std::cell::RefCell;
use crossbeam::sync::ArcCell;

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
        if s.is_empty() {
            (*GLOBAL_LOGGER.get()).clone()
        } else {
            s[s.len() - 1].clone()
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
