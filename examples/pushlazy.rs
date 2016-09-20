#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;

fn main() {

    let drain = slog_term::streamer().build().fuse();
    let log = Logger::root(drain, o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => "8dfljdf"));

    let log = log.new(o!("owned-fast-lazy" => {
        PushLazy(move |info : &Record, ser : ValueSerializer| {
            // no need for new allocations
            ser.serialize(info.file())
        })
    }));

    debug!(log, "debug"; "fast-lazy" =>
           PushLazy(move |info : &Record, ser : ValueSerializer| {
               // no need for new allocations
               ser.serialize(info.msg())
           })
    );

    trace!(log, "debug"; "drop-fast-lazy" =>
        PushLazy(move |_ : &Record, _ : ValueSerializer| {
            // drop of `ser` will emit unit (`()`/`void`) value
            Ok(())
        })
    );

}
