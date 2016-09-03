#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

const VERSION: &'static str = "0.1.0";

fn main() {

    let drain = slog_term::streamer().build();
    let log = Logger::root(drain, o!("version" => VERSION, "build-id" => "8dfljdf"));

    let log = log.new(o!("owned-fast-lazy" => {
        PushLazy(move |info : &Record, ser : ValueSerializer| {
            // no need for new allocations
            ser.serialize(info.file())
        })
    }));

    debug!(log, "debug", "fast-lazy" =>
           PushLazy(move |info : &Record, ser : ValueSerializer| {
               // no need for new allocations
               ser.serialize(&*info.msg())
           })
    );

    trace!(log, "debug", "drop-fast-lazy" =>
        PushLazy(move |_ : &Record, _ : ValueSerializer| {
            // drop of `ser` will emit unit (`()`/`void`) value
            Ok(())
        })
    );

}
