extern crate slog_example_lib;
extern crate slog_term;
#[macro_use]
extern crate slog;

use slog::DrainExt;

fn main() {
    let log = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    let lib = slog_example_lib::MyLib::init(Some(log));
    lib.do_the_thing();
}
