#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::IntoLogger;

fn main() {
    let log = slog_term::stderr().into_logger(o!("version" => "0.5"));

    trace!(log, "logging a trace message");
    debug!(log, "debug values", "x" => 1, "y" => -1);
    info!(log, "some interesting info", "where" => "right here");
    warn!(log, "be cautious!", "why" => "you never know...");
    error!(log, "something's wrong", "type" => "unknown");
    crit!(log, "abandoning test");
}
