#![cfg(feature = "std")]
use slog::Logger;
use std::io::{Error, ErrorKind};

#[test]
fn ok() {
    repro(slog::Logger::root(slog::Discard, slog::o!()))
}

fn repro(log: Logger) {
    let err = Error::new(ErrorKind::Other, "some error");
    slog::info!(log, "oops"; "err" => #err);
}
