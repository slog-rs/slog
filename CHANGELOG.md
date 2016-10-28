**Note:** `slog-*` sub-crates have their own ChangeLog files under
corresponding `crates/*/` directory.

# Change Log
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](http://keepachangelog.com/)
and this project adheres to [Semantic Versioning](http://semver.org/).

## 1.2.1 - 2016-10-27
### Added

* `OwnedKeyValueList::id` for owned key value unique identification

## 1.2.0 - 2016-10-21
### Changed

* **BREAKING**: `Serializer` takes `key : &'static str` now
* Fixed corner cases in `info!(...)` and other macros

## 1.1.0 - 2016-10-17
### Changed

* **BREAKING**: Rewrite handling of owned values.

## 1.0.1
### Changed

* Fix `use std` in `o!`
* Implement `fmt::Debug` for `Logger`

## 1.0.0 - 2016-09-21

First stable release.
