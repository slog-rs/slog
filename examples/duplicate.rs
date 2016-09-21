#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_json;
extern crate slog_stream;

use slog::DrainExt;

use std::io;

fn main() {
    let d1 = slog_term::streamer().stderr().full().build();
    let d2 = slog_stream::stream(io::stdout(), slog_json::new().add_default_keys().build());
    let log = slog::Logger::root(slog::duplicate(d1, d2).fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    trace!(log, "logging a trace message");
    debug!(log, "debug values"; "x" => 1, "y" => -1);
    info!(log, "some interesting info"; "where" => "right here");
    warn!(log, "be cautious!"; "why" => "you never know...");
    error!(log, "type" => "unknown"; "wrong {}", "foobar");
    crit!(log, "abandoning test");
}
