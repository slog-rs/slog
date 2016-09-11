#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

fn main() {
    let drain = slog_term::streamer().full().build();
    let log = Logger::root(
        drain.fuse(),
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
