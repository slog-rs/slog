#[macro_use]
extern crate slog;
extern crate slog_syslog;

use slog::Fuse;
use slog_syslog::Facility;


fn main() {
    let root = slog::Logger::root(slog_syslog::unix_3164(Facility::LOG_USER).fused(), o!("build-id" => "8dfljdf"));

    error!(root, "Test slog unix syslog message", "x" => -1);
}
