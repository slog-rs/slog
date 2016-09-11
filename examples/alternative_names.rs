#[macro_use(o,slog_log,slog_trace,slog_debug,slog_info,slog_warn,slog_error,slog_crit)]
extern crate slog;

extern crate slog_term;

use slog::DrainExt;

fn main() {
    let log = slog::Logger::root(slog_term::streamer().build().fuse(), o!());

    slog_trace!(log, "logging a trace message");
    slog_debug!(log, "debug values"; "x" => 1, "y" => -1);
    slog_info!(log, "some interesting info"; "where" => "right here");
    slog_warn!(log, "be cautious!"; "why" => "you never know...");
    slog_error!(log, "type" => "unknown"; "wrong {}", "foobar");
    slog_crit!(log, "abandoning test");
}
