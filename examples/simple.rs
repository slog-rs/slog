extern crate slog;

use slog::*;
use std::thread;

pub fn main() {
    let logger = Logger::root().add("module", "x").end();
    let sublogger = logger.new().add("index", 1).end();
    let thread_logger = sublogger.clone();

    let join = thread::spawn(move || {
        for i in 0..10 {
            if i % 2 == 0 {
                thread_logger.debug("subthread").add("iteration", i);
            } else {
                thread_logger.warning("subthread").add("iteration", i);
            }
            thread::sleep_ms(1000);
        }
    });

    sublogger.debug("start").add("should-appear-times", 1);
    thread::sleep_ms(1000);
    sublogger.debug("set_drain").add("to", "filter less than Info; duplicate").add("should-appear-times", 1);
    sublogger.set_drain(
        drain::FilterLevel::new(
            Level::Info,
            drain::Duplicate::new(
                drain::Streamer::new(std::io::stderr()),
                drain::Streamer::new(std::io::stdout()),
                )
            )
        );

    thread::sleep_ms(6000);
    sublogger.info("set_drain").add("to", "duplicate").add("should-appear-times", 2);
    sublogger.set_drain(
        drain::duplicate(
            drain::stream(std::io::stderr()),
            drain::stream(std::io::stdout()),
            )
        );
    thread::sleep_ms(1000);
    sublogger.info("join");

    join.join().unwrap();
    sublogger.error("exit");
}

