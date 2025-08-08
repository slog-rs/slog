//! A set of common imports needed by most programs that use `slog`.
//!
//! It is intended to be used like follows:
//! ```
//! use slog::prelude::*;
//! fn my_func(logger: &Logger, x: i32) {
//!     info!(logger, "slog rules!"; "x" => x);
//!     if x < 0 {
//!         warn!(logger, "negative numbers are scary"; "x" => x);
//!     }
//! }
//! ```
//!
//! This includes the logging macros ([`log!`](crate::log), [`trace!`](crate::trace), ...) and [`slog::Logger`](crate::Logger).
//! It also includes [`slog::Serde`] and [`slog::FnValue`], as those are frequently useful as well.
//!
//! Adding new items here is a breaking change,
//! because it can cause conflicts with other bulk-imported modules.

#[cfg(feature = "nested-values")]
pub use crate::Serde;
pub use crate::{crit, debug, error, info, log, trace, warn};
pub use crate::{FnValue, Logger};
