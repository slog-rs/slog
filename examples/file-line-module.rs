#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;
use slog::drain::{IntoLogger};

fn main() {
    let drain = slog_term::async_stderr();
    let log = drain.into_logger(o!("place" =>
        move |info : &Record| {
            format!("{}:{} {}",
                    info.file(),
                    info.line(),
                    info.module(),
            )
        }
    ));

    debug!(log, "HERE");
}
