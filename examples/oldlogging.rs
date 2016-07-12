#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use slog::*;

fn main() {
    let log = Logger::new_root(o!("version" => "0.5"));
    log.set_drain(slog_term::stderr());
    slog_stdlog::set_logger(log.clone()).unwrap();

    info!("standard logging redirected to slog");
}
