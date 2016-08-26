#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_stdlog;
#[macro_use]
extern crate log;

use slog::drain::IntoLogger;

fn bar() {
    info!("inside bar!");

}
fn foo() {
    info!("inside foo!");

    slog_stdlog::scope(
        slog_stdlog::with_current_logger(|l| l.new(o!("subfunction2" => "bar"))),
        || bar()
    )
}

fn main() {
    let log = slog_term::stderr().into_logger(o!("version" => "0.5"));
    slog_stdlog::set_logger(log.clone()).unwrap();

    info!("standard logging redirected to slog");

    slog_stdlog::scope(
        slog_stdlog::with_current_logger(|l| l.new(o!("subfunction" => "foo"))),
        || {
            info!("entering foo!");
            foo();
            info!("left foo!");
        }
    )
}
