#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use slog::IntoLogger;

fn main() {
    let log = slog_term::stderr().into_logger(o!("version" => "0.5"));
    slog_stdlog::set_logger(log.clone()).unwrap();

    info!("standard logging redirected to slog");
}
