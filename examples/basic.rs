extern crate slog;

use slog::*;
use std::thread;

fn main() {
    let root = Logger::root().add("example", "basic").end();
    let log = root.new().add("thread-name", "main").end();
    let tlog = root.new().add("thread-name", "sleep1000").end();

    log.set_drain(
        drain::duplicate(
            drain::filter_level(Level::Info, drain::stream(std::io::stderr())),
            drain::stream(std::io::stdout()),
            )
        );

    let join = thread::spawn(move || {
        tlog.info("subthread started");
        thread::sleep_ms(1000);
        tlog.info("subthread finished");
    });

    let time_ms = 10000;
    log.info("sleep").add("time", time_ms);
    thread::sleep_ms(time_ms);

    log.info("join");

    join.join().unwrap();
    log.warning("exit");
}

