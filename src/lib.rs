//! # Slog -  Structured, extensible, composable logging for Rust
//!
//! `slog-rs` is an ecosystem of reusable components for structured, extensible,
//! composable logging for Rust.
//!
//! `slog` is `slog-rs`'s main crate providing core components shared between
//! all other parts of `slog-rs` ecosystem.
//!
//! This is automatically generated technical documentation of `slog`. For
//! information about project organization, development, help, etc. please see
//! [slog github page](https://github.com/slog-rs/slog)
//!
//! ## Core advantages over `log` crate
//!
//! * **extensible** - `slog` crate provides core functionality: very basic
//!   and portable standard feature-set based on open `trait`s that allow
//!   implementing new features that can be independently published.
//! * **composable** - `trait`s that `slog` exposes to provide extensibility
//!   are designed to be easy to efficiently reuse and combine, using Rust type
//!   system. Due to this it's possible to combine functions to allow every
//!   application to carefully specify how, when and where to log information.
//! * **structured** - Logging with `slog` is not based on just one global
//!   logger. `slog`'s `Logger`s carry hierarchy of key-value data that contains
//!   the context of logging - information that otherwise would have to be
//!   repeated in every logging statement.
//! * both **human and machine readable** - By keeping the key-value data format
//!   and retaining it's type information, meaning of logging data is preserved.
//!   Data can be serialized to machine readable formats like JSON and send it
//!   to data-mining system for further analysis etc. On the other hand, when
//!   presenting on screen, logging data can be shown in eastetically pleasing
//!   and easy to understand way.
//!
//! ## `slog` features
//!
//! * performance oriented; read [what makes slog
//!   fast](https://github.com/slog-rs/slog/wiki/What-makes-slog-fast) and see:
//!   [slog bench log](https://github.com/dpc/slog-rs/wiki/Bench-log)
//!   * lazily evaluation through closure values
//!   * async IO support included: see [`slog-extra`
//!     crate](https://docs.rs/slog-extra) * `#![no_std]` support (with opt-out
//!     `std` cargo feature flag)
//! * hierarchical loggers
//! * modular, lightweight and very extensible
//!   * tiny core crate that does not pull any dependencies
//!   * feature-crates for specific functionality
//!   * using `slog` in library does not force users of the library to use slog
//!     (but provides additional functionality); see [example how to use
//!     `slog` in library](https://github.com/slog-rs/example-lib)
//! * backward and forward compatibility with `log` crate:
//!   see [`slog-stdlog` crate](https://docs.rs/slog-stdlog)
//! * convieniance crates:
//!   * logging-scopes for implicit `Logger` passing: see
//!     [slog-scope](https://docs.rs/slog-scope)
//! * many existing core&community provided features:
//!   * multiple outputs
//!   * filtering control
//!     * compile-time log level filter using cargo features (same as in `log`
//!       crate)
//!     * by level, msg, and any other meta-data
//!     * [`slog-envlogger`](https://github.com/slog-rs/envlogger) - port of
//!       `env_logger` * terminal output, with color support: see [`slog-term`
//!       crate](docs.r/slog-term)
//!  * [json](https://docs.rs/slog-json)
//!    * [bunyan](https://docs.rs/slog-bunyan)
//!  * [syslog](https://docs.rs/slog-syslog)
//!    and [journald](https://docs.rs/slog-journald) support
//!  * run-time configuration:
//!    * run-time behavior change;
//!      see [slog-atomic](https://docs.rs/slog-atomic)
//!    * run-time configuration; see [slog-config](https://docs.rs/slog-config)
//!
//!
//! [signal]: https://github.com/slog-rs/misc/blob/master/examples/signal.rs
//! [env_logger]: https://crates.io/crates/env_logger
//!
//! ## Notable details
//!
//! **Note:** `slog` by default removes at compile time trace and debug level
//! statements in release builds, and trace level records in debug builds. This
//! makes `trace` and `debug` level logging records practically free, which
//! should encourage using them freely. If you want to enable trace/debug
//! messages or raise the compile time logging level limit, use the following in
//! your `Cargo.toml`:
//!
//! ```norust
//! slog = { version = ... ,
//!          features = ["max_level_trace", "release_max_level_warn"] }
//! ```
//!
//! Due to the `macro_rules` limitation log macros syntax comes in several
//! versions. See `log!` macro, and pay attention to `;` and `,`
//! details.
//!
//! Root drain (passed to `Logger::root`) must be one that does not ever return
//! errors, which forces user to pick error handing strategy. You can use
//! `DrainExt::fuse()` or `DrainExt::ignore_err()` methods from `DrainExt` to do
//! it conveniently.
//!
//! [signal]: https://github.com/slog-rs/misc/blob/master/examples/signal.rs
//! [env_logger]: https://crates.io/crates/env_logger
//! [fn-overv]: https://github.com/dpc/slog-rs/wiki/Functional-overview
//! [atomic-switch]: https://docs.rs/slog-atomic/0.4.3/slog_atomic/
//!
//! ## Where to start
//!
//! [`Drain`](trait.Drain.html), [`Logger`](struct.Logger.html) and
//! [`log` macro](macro.log.html) are the most important elements of
//! slog.
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
//! Visit [slog-rs gitter channel](https://gitter.im/slog-rs/slog) for immediate
//! help.

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

#[cfg(not(feature = "std"))]
use alloc::arc::Arc;
#[cfg(not(feature = "std"))]
use alloc::boxed::Box;
#[cfg(not(feature = "std"))]
use alloc::rc::Rc;
#[cfg(not(feature = "std"))]
use collections::string::String;

use core::fmt;

use core::result;
use core::str::FromStr;

#[cfg(feature = "std")]
use std::boxed::Box;
#[cfg(feature = "std")]
use std::rc::Rc;
#[cfg(feature = "std")]
use std::string::String;
#[cfg(feature = "std")]
use std::sync::Arc;


/// This type is here just to abstract away lack of `!` type support in stable
/// rust during time of the release. It will be switched to `!` at some point
/// and `Never` should not be considered "stable" API.
#[doc(hidden)]
pub type Never = ();


/// This is not part of "stable" API
#[doc(hidden)]
pub static STATIC_TERMINATOR_UNIT: () = ();

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
include!("_ser.rs");
include!("_compat.rs");

#[cfg(test)]
mod tests;
