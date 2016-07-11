#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::*;

fn main() {
    let log = Logger::new_root(o!("version" => "0.5"));
    log.set_drain(slog_term::stderr());

    log.trace("logging a trace message", b!());
    log.debug("debug values", b!("x" => 1, "y" => -1));
    log.info("some interesting info", b!("where" => "right here"));
    log.warn("be cautious!", b!("why" => "you never know..."));
    log.error("something's wrong", b!("type" => "unknown"));
    log.critical("abandoning test", b!());
}
