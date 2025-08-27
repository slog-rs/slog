//! Tests that slog respects `#[track_caller]`
//!
//! WARNING: Formatting changes to this file will disrupt the line and column numbers,
//! triggering test failures.
//! You have been warned.

use std::panic::Location;

// NOTE: Using a path attribute here breaks Location.file()
mod separate_file;

#[test]
#[rustversion::attr(before(1.79), ignore = "no const Location::caller")]
fn track_caller() {
    let record = separate_file::record_track();
    assert_eq!(record.file(), Location::caller().file());
    assert_eq!(record.line(), 12);
    assert_eq!(record.column(), 5);
}

#[test]
fn no_track_caller() {
    let record = separate_file::record_no_track();
    assert_eq!(record.file(), "tests/track_caller/separate_file.rs");
    assert_eq!(record.line(), 7);
    assert_eq!(record.column(), 18);
}

#[test]
fn track_caller_loc() {
    let loc = const { indirect() };
    assert_eq!(loc.line(), 31)
}

#[track_caller]
const fn indirect() -> &'static core::panic::Location<'static> {
    Location::caller()
}