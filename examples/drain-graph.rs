#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_stream;
extern crate slog_syslog;

use slog::*;

use std::fs::File;

fn main() {
    let file = File::create("/tmp/myloggingfile").unwrap();
    let stream = slog_stream::stream(file, slog_json::new().build());
    let syslog = slog_syslog::unix_3164(slog_syslog::Facility::LOG_DAEMON);
    let root = Logger::root(Duplicate::new(
            LevelFilter::new(stream, Level::Info),
            LevelFilter::new(syslog, Level::Warning),
            ).fuse(), o!());
    info!(root, "We did it!");
}
