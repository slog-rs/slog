# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## [Unreleased]

## 2.8.1 - 2025-10-05
This fixes an accidental breaking change in the v2.8.0 release,
where the public API was changed from `erased_serde v0.4` to `erased_serde v0.3`.
This should not affect normal users of the API but could break drains supporting `nested-values`.

I (@Techcable) apologize for this breakage and have yanked the 2.8.0 release due to this issue. I will try to setup [`cargo-semver-checks`] in the CI to avoid this in the future.

[`cargo-semver-checks`]: https://github.com/obi1kenobi/cargo-semver-checks

### Fixed
- Restore compatibility with `erased-serde` version `0.3`.
  - Compatibility with `erased-serde` version `0.4` may be provided again in a
    future version, but will be done so in a backwards compatible way.
  - Thanks to @demurgos for recognizing and fixing this in PR #359
- Ensure compatibility with `-Zminimal-versions` flag by specifying minimal dependency versions (fixes #357)
  - Not (yet) tested in CI

## [2.8.0] - 2025-10-05
This is the biggest slog release since 2.0. It fully preserves compatibility with prior releases.

### Highlights
- Enables the `nested-values` feature by default.
  The next major release of `slog_json`, `slog_term`, etc should also enable this feature by default.
- Added `Logger::flush` and `Drain::flush` methods.
- Improved support for logging errors
  - Have `#` log specifier transparently support `&` references to error
  - Implement `slog::Value` for `Box<dyn std::error::Error>` and `anyhow::Error`
- Define a `prelude` module for common imports
- Upgraded to Rust 2018 and require Rust 1.61
  - Deprecate the old `slog_log!` macros in favor of `slog::log!`
- Many internal improvements & bug fixes

### Changed
* Enables the `nested-values` feature by default.
* Updated to Rust 2018
  * NOTE: Old 2015 crates will still work because of [excellent 2015/2018 compatibility](https://blog.rust-lang.org/2018/07/27/what-is-rust-2018.html#managing-compatibility)
* Bump MSRV to 1.61
  - This is already required for `erased-serde`, `syn`, `serde_core`, and many other important crates in the rust ecosystem.
* Update `erased-serde` from v0.3 to v0.4
* Depend on [`serde_core`] rather than `serde` to reduce compile times.

### Added
* Added a `Logger::flush` and `Drain::flush` methods
  * Since this is a new method, not all drains implement it.
    Calling it on an unsupported drain will give an error
* Define a `prelude` module for common imports.
   Replace `use slog::{Serde, Logger, info, debug, trace}` with `use slog::prelude::*`
* Optionally implement Drain for [`parking_lot::Mutex`].
  This is noticeably faster than `std::sync::Mutex`, is smaller, and avoids poisoning.
  * This feature has a separate name per version to allow supporting multiple versions of `parking_lot` at once. The current version (v0.12) has feature name `parking_lot_0_12`
* Implement slog::Value for `dyn std::error::Error` and `Box<dyn std::error:Error` (along with the `Send`/`Sync` versions).
* Optionally implement slog::Value for [`anyhow::Error`] (requires `anyhow` feature)
* Add `ErrorRef` wrapper to enable logging error references (PR #327)
  * The `#` error formatter in macros was updated to automatically select `ErrorValue` or `ErrorRef` (PR #328)
* Add `emit_bytes` method (PR #290)
  * Control formatting using `BytesKind` enum
* Implement `Value` for `std::borrow::Cow`

### Deprecated
* Deprecate old prefixed macros like `slog_log`.
  Rust 2018 macro paths like `slog::log!` make these unnecessary.

### Fixed
* Fix logging of references to errors (resolved by introduction of `ErrorRef` in PR #327)
* Take advantage of the fact that in 1.81 [`std::error::Error`] has been moved to `core`.
  On this rust version, slog now supports the error trait without requiring `std`.
* Add `#[must_use]` to most public types, including `slog::Logger`, `slog::OwnedKV`, and `slog::FnValue`.
* Give a descriptive error when the `nested-values` feature is unsupported.
* Alias the `slog::Never` type to [`std::convert::Infallible`], fixing issue #209.
* doc: Fix typo in slog::Value docs (issue #335)
  * Thank you to @larswirzenius for noticing this!
* doc: Fix "lazy continuation" in `slog::Logger` doc
  * This mistake was caught by a new lint [`#[warn(clippy::doc_lazy_continuation)]`](https://rust-lang.github.io/rust-clippy/rust-1.81.0/index.html#/doc_lazy_continuation)
* Fix some internal warnings
  * Example: Avoid the new [`#[warn(unexpected_cfgs)]`](https://blog.rust-lang.org/2024/05/06/check-cfg.html) lint.
  * The crate almost passes clippy::pedantic now, except for a few very verbose lints like `clippy::must-use-candidate`
  * None of these changes should affect user crates
* Stop requiring the `serde_derive` crate to implement the `nested-values` feature.
  We depend on the new `serde_core` instead of even requiring `serde`.
* The `nested-values` feature no longer requires `serde/std` (was a mistake).
* Always support 128-bit integers, even on the `wasm32-unknown-emscripten` target.
* Fix support for `feature="nothreads"`
  * Internal refactoring to make different feature combos much easier (PR #301)
* Switch from Travis CI to Github Actions (fixes #294)
  * `rustfmt --check` now run by default
  * Require `clippy` to succeed without warnings
  * Require all feature combinations to pass tests.
    * Make exception for `nested-values` feature on MSRV (1.49),
      because `erased-serde` dependency requires `edition="2021"` (needs 1.56)
* Fix `#` format when not used as a last argument.
* Fix duplicate `AsRef<str>` implementations when using `dynamic-keys` feature.
* Fix incorrect 'long' name for `slog::Level::Warning` (fixes issue #282)

[`std::error::Error`]: https://doc.rust-lang.org/std/error/trait.Error.html
[`std::convert::Infallible`]: https://doc.rust-lang.org/std/convert/enum.Infallible.html
[`anyhow::Error`]: https://docs.rs/anyhow/1/anyhow/struct.Error.html
[`serde_core`]: https://docs.rs/serde_core/1/serde_core/
[`parking_lot::Mutex`]: https://docs.rs/parking_lot/latest/parking_lot/type.Mutex.html

## 2.7.0 - 2020-11-29

* Implement `Hash` for `Level` and `FilterLevel`
* Add #% for alternate display of the value part
* Implement `Eq` for dynamic `Key`s
* Add `emit_error` to `Serializer`, `#` for serializing foreign errors, and
  `impl Value for std::io::Error`
* Add `Serde` (if `nested-values` feature is enabled) to optionally wrap a value
  that implements [serde::Serialize] so it's written to the log record as an
  object, rather than a primitive.

## 2.6.0 - 2019-10-28

* Add #? for pretty-debug printing the value part

## 2.5.3 - ????-??-??

* Use fully qualified call syntax for `Logger::log` in macros

## 2.5.2 - 2019-07-22

* Restored parsing of `Level` and `FilterLevel` truncated names

## 2.5.1 - 2019-07-11

* Added parsing of `Level` and `FilterLevel` short names

## 2.5.0 - 2019-07-11

* Added `FilterLevel::accepts`
* Added `as_str`, `as_short_str` and `Display` to `FilterLevel`

## 2.4.1 - 2018-10-03

* disable support for i128/u128 types if rustc is old

## 2.4.0 - 2018-09-19

* Implement Value for 128 bit integers
* Add support 2018-style macro imports
  * **WARNING**: This is a breaking change that we couldn't avoid. Users using
    explicitly macro import (like `#[macro_use(slog_o)]`) must add
    `__slog_builtin` to the import list.
* Bump minimum supported Rust version to 1.26

## 2.3.3 - 2018-07-20

* `impl Value for SocketAddr`

## 2.3.2 - 2018-07-20

* Revert broken changes:
  * Make `?` and `%` formatters in `kv!` more flexible
  * Export local inner macros to help with Rust 2018 testing

## 2.3.0 - 2018-07-20

* Export local inner macros to help with Rust 2018 testing
* Stabilize `Record::new`
* Make `?` and `%` formatters in `kv!` more flexible

## 2.2.3 - 2018-03-28

* Fix (again) problems introduced by `2.2.1`

## 2.2.2 - 2018-03-26

* Fix problems introduced by `2.2.1`

## 2.2.1 - 2018-03-24

* Add `is_x_enabled()` for queering (imprecise) log-level

## 2.2.0 - 2018-02-13
### Added

* Support for named format arguments in format messages. They will now become
  respectively named key-value pairs.

## 2.1.0 - 2017-12-10
### Added

* Support for nested-values through `emit_serde`, behind `nested-values` feature flag,
  disabled by default for backward compatibility. **Note**: Consider unstable for the time
  being.
* Support for dynamic key (`String` vs `&'static str`), behind `dynamic-keys` feature
  flag, disabled by default for backward compatibility. **Note**: Consider unstable for
  the time being.

## 2.0.12 - 2017-09-14
### Changed

* `#[allow(dead_code)` on unused log statements

## 2.0.11 - 2017-09-13
### Changed

* Impl `Value` for `std::path::Display`

## 2.0.10 - 2017-09-09
### Changed

* Remove unnecessary 'static bound on `FnValue`

## 2.0.9 - 2017-08-23
### Changed

* Update README

## 2.0.6 - 2017-05-27
### Changed

* Fix for https://github.com/rust-lang/rust/pull/42125

## 2.0.5 - 2017-05-15
### Changed

* Relicense under MPL/Apache/MIT

## 2.0.4 - 2017-05-05
### Fixed

* Documentation improvements

## 2.0.3 - 2017-05-05
### Fixed

* Documentation fixes

## 2.0.2 - 2017-04-12
### Fixed

* Compile time logging level filtering

## 2.0.0 - 2017-04-11
### Changed (since v1; bigger picture)

* Unified and simplified logging macros structure and ordering.
* Added logging Record `tags`.
* Refactored key-value pairs traits and structures and overall handling.
  * It's now possible to `impl KV for MyStruct`.
  * `kv!` can be used to create on stack key-value list.
  * `KV`-implementing data can appear on the key-value list directly.
* Support chaining of `OwnedKVList`s. Existing `Logger` can be used as a `Drain`
  to allow custom handling logic for a part of the logging hierarchy.
* Added associated `Ok` type to `Drain`.
* Support for `Drain`s unwind safety.
* Refactored `Record` to optimize performance on async operations.
* `slog-extra` has been renamed to `slog-async` since that's the only functionality it contained.
* `slog-stream` is obsoleted and won't be used in `slog v2` at all. It was a wrong abstraction.
  `Decorators` functionality was moved to `slog-term`.
* `slog-term` provides open `Decorator` traits to allow multiple terminal / file writing backends.
* `slog-term` default `Decorator`s use `term` crate and should work correctly on all supported OSes.
* `DrainExt` has been removed and utility methods moved directly to `Drain`
* `slog-stdlog` utilizes `slog-scope` directly.
* Support for "typed" `Logger`s to allow squeezing last drops of performance possible,
  at the cost of `T` in `Logger<T>`.

## 2.0.0-3.1 - 2017-03-25
### Added

* Support for `fmt::Display` values with `%` in `kv!`


## 2.0.0-3.0 - 2017-03-25
### Changed

* Added support for own `KV` and `Value` implementations
* Streamlined the formatting syntax for `log!` and friends; **BREAKING**
* Lazy values need explicit `FnValue` wrapper; **BREKING**

### Added

* `kv!` macro

## 2.0.0-2.2 - 2017-03-19
### Fixes

* Bunch of trait-related fixes

## 2.0.0-2.1 - 2017-03-11
### Fixed

* Require `MapErr` and `Filter` to be `UnwindSafe`

## 2.0.0-2.0 - 2017-03-11
### Changed

* Make `Logger::root` return "erased" version
* Introduce `Logger::root_typed` for "non-erased" `Logger` creation

## 2.0.0-1.0 - 2017-02-23

### Fixed

* `fmt::Debug` for `MutexDrainError`

### Changed
* Parametrize `Logger` over the `Drain` it holds and introduce "erased" version
* Enforcing `UnwindSafe` `Drain`s for `Logger`s
* Refactored key-value pairs traits and structures
* Renamed some types
* Support chaining of `OwnedKVList`s
* Added associated `Ok` type to `Drain`
* Refactored `Record` to optimize performance on async
  operations
* Minimal rustc version required: `1.15.0`
* `DrainExt` has been removed and utility methods moved directly to `Drain`

### Added

* Macros to create `OwnedKV` and `BorrowedKV`
* `Logger` implements `Drain`

## 1.5.0 - 2017-01-19
### Changed

* Order of key-value pairs is now strictly defined

### Added

* `Logger` implements `Drain`

### Deprecated

* Creation of `OwnedKeyValueList`

## 1.4.1 - 2017-01-19
### Fixed

* Fix an invalid syntax exposed by nightly rust change (Issue #103)

## 1.4.0 - 2016-12-27
### Changed

* Updated documentation

### Deprecated

* `OwnedKeyValueList::id`

## 1.3.2 - 2016-11-19
### Added

* `slog_o` as an alternative name for `o`

## 1.3.1 - 2016-11-19
### Fixed

* Cargo publishing mistake.

## 1.3.0 - 2016-10-31
### Changed

* **BREAKING**: Removed default `Send+Sync` from `Drain`

## 1.2.1 - 2016-10-27
### Added

* `OwnedKeyValueList::id` for owned key value unique identification

## 1.2.0 - 2016-10-21
### Changed

* **BREAKING**: `Serializer` takes `key : &'static str` now

### Fixed

* Corner cases in `info!(...)` and other macros

## 1.1.0 - 2016-10-17
### Changed

* **BREAKING**: Rewrite handling of owned values.

## 1.0.1
### Fixed

* `use std` in `o!`

### Added

* Implement `fmt::Debug` for `Logger`

## 1.0.0 - 2016-09-21

First stable release.
