#[macro_use]
extern crate slog;

use slog::*;
use std::thread;

use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Duration;

const VERSION : &'static str = "0.1.0";

fn slow_fib(n : u64) -> u64 {
    match n {
        0|1|2 => 1,
        n => slow_fib(n-1) + slow_fib(n-2),
    }
}

fn main() {
    // Create a new group of loggers, sharing one drain.
    //
    // Note `v!` macro for more natural `key-value` pair
    // building.
    let root = Logger::new_root(v!("version" => VERSION, "build-id" => "8dfljdf"));

    // Create child loggers from existing ones. Children
    // clone `key: value` pairs from their parents.
    //
    // Build logging context as data becomes available.
    let log = root.new(v!("child" => 1));

    // Closures can be used for values that change at runtime.
    // Data captured by the closure needs to be `Send+Sync`.
    let counter = Arc::new(AtomicUsize::new(0));
    let log = log.new(v!("counter" => {
        let counter = counter.clone();
        /// Note the `move` to capture `counter`,
        /// and unfortunate `|_ : &_|` that helps
        /// current `rustc` limitations. In the future,
        /// a `|_|` could work.
        move |_ : &_| { counter.load(SeqCst)}
    }));

    log.info("before-fetch-add", s!()); // counter == 0
    counter.fetch_add(1, SeqCst);
    log.info("after-fetch-add", s!()); // counter == 1

    // Drains can be swapped atomically (race-free).
    log.set_drain(
        // drains are composable
        drain::filter_level(
            Level::Info,
            drain::stream(
                std::io::stderr(),
                // multiple outputs formats are supported
                format::Json::new(),
                ),
            ),
        );

    // Closures can be used for lazy evaluation:
    // This `slow_fib` won't be evaluated, as the current drain discards
    // "trace" level logging records.
    log.debug("debug", s!("lazy-closure" => |_ : &_| slow_fib(40)));

    // Loggers are internally atomically reference counted so can be cloned,
    // passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {
            log.info("subthread", s!("stage" => "start"));
            thread::sleep(Duration::new(1, 0));
            log.info("subthread", s!("stage" => "end"));
        }
    });

    join.join().unwrap();
}
