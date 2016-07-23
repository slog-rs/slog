#[macro_use]
extern crate slog;
extern crate slog_syslog;

use slog::drain::IntoLogger;
use slog_syslog::Facility;

fn main() {
    let root = slog_syslog::unix_3164(
        Facility::LOG_USER,
    ).into_logger(o!("build-id" => "8dfljdf"));

    root.error("Test slog unix syslog message", b!("x" => -1));
}
