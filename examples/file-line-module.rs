#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

fn main() {
    let drain = slog_term::async_stderr();
    let log = Logger::root(
        drain,
        o!("place" =>
           move |info : &Record| {
               format!("{}:{} {}",
                       info.file(),
                       info.line(),
                       info.module(),
                       )
           }
          )
        );

    debug!(log, "HERE");
}
