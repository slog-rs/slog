#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::*;

fn main() {
    let log = Logger::new_root(o!("version" => "9.7"));
    log.set_drain(slog_term::stderr());

    log.trace("message", b!("a" => "trace"));
    log.debug("message", b!("b" => "debug"));
    log.info("message", b!("c" => "info"));
    log.warn("message", b!("d" => "warn"));
    log.error("message", b!("e" => "error"));
    log.critical("message", b!("f" => "critical"));
}
