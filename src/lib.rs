//! # Slog -  Structured, composable logging for Rust
//!
//! ## Features
//!
//! * easy to use
//! * great performance; see: [slog bench log](https://github.com/dpc/slog-rs/wiki/Bench-log)
//! * `#![no_std]` support (with opt-out `std` cargo feature flag)
//! * hierarchical loggers
//! * lazily evaluated values
//! * modular, lightweight and very extensible
//! 	* tiny core crate that does not pull any dependencies
//! 	* feature-crates for specific functionality
//! * backward compatibility for standard `log` crate (`slog-stdlog` crate)
//! 	* supports logging-scopes
//! 	* using slog in library does not force users of the library to use slog
//! 	  (but gives benefits); see `crates/example-lib`
//! * drains & output formatting
//! 	* filtering
//! 		* compile-time log level filter using cargo features (same as in `log` crate)
//! 		* by level, msg, and any other meta-data
//! 		* [`slog-envlogger`](https://github.com/slog-rs/envlogger) - port of `env_logger`
//! 	* multiple outputs
//! 	* asynchronous IO writing
//! 	* terminal output, with color support (`slog-term` crate)
//! 	* Json (`slog-json` crate)
//! 		* Bunyan (`slog-bunyan` crate)
//! 	* syslog (`slog-syslog` crate)
//! 	* first class custom drains
//!
//! ## Advantages over `log` crate
//!
//! * **extensible** - `slog` provides core functionality, and some standard
//!   feature-set. But using traits, anyone can easily implement as
//!   powerful fully-custom features, publish separately and grow `slog` feature-set
//!   for everyone.
//! * **composable** - Wouldn't it be nice if you could use
//!   [`env_logger`][env_logger], but output authentication messages to syslog,
//!   while reporting errors over network in json format? With `slog` drains can
//!   reuse other drains! You can combine them together, chain, wrap - you name it.
//! * **context aware** - It's not just one global logger. Hierarchical
//!   loggers carry information about context of logging. When logging an error
//!   condition, you want to know which resource was being handled, on which
//!   instance of your service, using which source code build, talking with what
//!   peer, etc. In standard `log` you would have to repeat this information in
//!   every log statement. In `slog` it will happen automatically. See
//!   [slog-rs functional overview page][functional-overview] to understand better
//!   logger and drain hierarchies and log record flow through them.
//! * both **human and machine readable** - By keeping the key-value data format,
//!   meaning of logging data is preserved. Dump your logging to a JSON file, and
//!   send it to your data-mining system for further analysis. Don't parse it from
//!   lines of text anymore!
//! * **lazy evaluation** and **asynchronous IO** included. Waiting to
//!   finish writing logging information to disk, or spending time calculating
//!   data that will be thrown away at the current logging level, are sources of big
//!   performance waste. Use `AsyncStreamer` drain, and closures to make your logging fast.
//! * **run-time configuration** - [`AtomicSwitch`][atomic-switch] drain allows
//!   changing logging behavior in the running program. You could use eg. signal
//!   handlers to change logging level or logging destinations. See
//!   [`signal` example][signal].
//!
//! ## Notable details
//!
//! `slog` by default removes at compile time trace and debug level statements
//! in release builds, and trace level records in debug builds. This makes
//! `trace` and `debug` level logging records practically free, which should
//! encourage using them freely. If you want to enable trace/debug messages
//! or raise the compile time logging level limit, use the following in your
//! `Cargo.toml`:
//!
//! ```norust
//! slog = { version = "1.2", features = ["max_level_trace", "release_max_level_warn"] }
//! ```
//!
//! Due to the `macro_rules` limitation log macros syntax comes in several
//! versions. See `log!` macro, and pay attention to `;` and `,`
//! details.
//!
//! Root drain (passed to `Logger::root`) must be one that does not ever
//! return errors, which forces user to pick error handing strategy. You
//! can use `.fuse()` or `.ignore_err()` methods from `DrainExt` to do
//! it conveniently.
//!
//! [signal]: https://github.com/slog-rs/misc/blob/master/examples/signal.rs
//! [env_logger]: https://crates.io/crates/env_logger
//! [functional-overview]: https://github.com/dpc/slog-rs/wiki/Functional-overview
//! [atomic-switch]: https://docs.rs/slog-atomic/0.4.3/slog_atomic/
//!
//! ## Examples & help
//!
//! Look at [slog-rs examples in `slog-misc`
//! repository](https://github.com/slog-rs/misc/tree/master/examples)
//!
//! Read [slog-rs wiki pages](https://github.com/slog-rs/slog/wiki)
//!
//! Check sources of other [software using
//! slog-rs](https://crates.io/crates/slog/reverse_dependencies)
//!
//! Visit [slog-rs gitter channel](https://gitter.im/slog-rs/slog) for immediate help.

#![cfg_attr(not(feature = "std"), feature(alloc))]
#![cfg_attr(not(feature = "std"), feature(collections))]
#![warn(missing_docs)]

#![no_std]

#[macro_use]
#[cfg(feature = "std")]
extern crate std;
#[cfg(not(feature = "std"))]
extern crate alloc;
#[cfg(not(feature = "std"))]
extern crate collections;

use core::str::FromStr;
use core::fmt;
use core::result;

#[cfg(feature = "std")]
use std::sync::Arc;
#[cfg(not(feature = "std"))]
use alloc::arc::Arc;


#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;

/// This type is here just to abstract away lack of `!` type support in stable
/// rust during time of 1.0 release. It will be switched to `!` at some point
/// and `Never` should not be considered "stable" API.
#[doc(hidden)]
pub type Never = ();


/// Serialization
pub mod ser;

pub use ser::{ValueSerializer, Serializer, Value, KV};

// One main namespace is enough, but for conveniance break
// the code into parts.
//
// Use include so we don't have to deal with privacy between modules.
include!("_record.rs");
include!("_macros.rs");
include!("_kv.rs");
include!("_level.rs");
include!("_logger.rs");
include!("_drain.rs");

#[cfg(test)]
mod tests;
