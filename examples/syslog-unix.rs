#[macro_use]
extern crate slog;
extern crate slog_syslog;

use slog::*;
use slog_syslog::Facility;

fn main() {
    let root = Logger::new_root(o!("build-id" => "8dfljdf"));
    root.set_drain(
        slog_syslog::unix_3164(
            Facility::LOG_USER,
        )
    );

    root.error("Test slog unix syslog message", b!("x" => -1) );

}
