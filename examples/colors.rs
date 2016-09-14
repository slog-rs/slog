#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

fn main() {
    let log = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    trace!(log, "logging a trace message");
    debug!(log, "debug values"; "x" => 1, "y" => -1);
    info!(log, "some interesting info"; "where" => "right here");
    warn!(log, "be cautious!"; "why" => "you never know...");
    error!(log, "type" => "unknown"; "wrong {}", "foobar");
    crit!(log, "abandoning test");
}
