#[macro_use(o,slog_log,slog_trace,slog_debug,slog_info,slog_warn,slog_error,slog_crit)]
extern crate slog;

extern crate slog_term;

use std::time::{Duration, SystemTime};


use slog::DrainExt;

fn main() {
    let now = SystemTime::now();
    let streamer = slog_term::StreamerBuilder::new()
        .use_custom_timestamp(move |io| {
            let elapsed = now.elapsed().unwrap();
            write!(io, "{:5}.{:06}", elapsed.as_secs(), elapsed.subsec_nanos()/1000)
            });
    let log = slog::Logger::root(streamer.build().fuse(), o!());

    slog_trace!(log, "logging a trace message");
    std::thread::sleep(Duration::from_millis(250));
    slog_debug!(log, "debug values"; "x" => 1, "y" => -1);
    std::thread::sleep(Duration::from_millis(250));
    slog_info!(log, "some interesting info"; "where" => "right here");
    std::thread::sleep(Duration::from_millis(250));
    slog_warn!(log, "be cautious!"; "why" => "you never know...");
    std::thread::sleep(Duration::from_millis(250));
    slog_error!(log, "type" => "unknown"; "wrong {}", "foobar");
    std::thread::sleep(Duration::from_millis(250));
    slog_crit!(log, "abandoning test");
}
