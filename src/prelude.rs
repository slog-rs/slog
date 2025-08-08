//! A set of common imports needed by most programs that use `slog`.
//!
//! It is intended to be used like follows:
//! ```
//! use slog::prelude::*;
//! fn my_func(logger: &Logger, x: i32) -> {
//!     info!(logger, "slog rules!"; x => x);
//!     if x < 0 {
//!         warn!(logger, "negative numbers are scarry"; x => x);
//!     }
//! }
//! ```
//!
//! This currently includes the logging macros ([`log!`](crate::log), [`trace!`](crate::trace), ...) and [`slog::Logger`](crate::Logger).
//!
//! Adding new items here is a breaking change,
//! because it can cause conflicts with other bulk-imported modules,

pub use crate::Logger;
pub use crate::{
    log,
    trace,
    debug,
    info,
    warn,
    error,
    crit,
};
/*
 * TODO: Should these be included?
 * use slog::{FnValue, PushFnValue, Serde};
 */