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
    let root = root_logger!("version" => VERSION);

    // Child loggers clone the `key: values` pairs from their parents.
    let _log = child_logger!(root, "child" => 1);

    // Closures can be used for values that change at runtime.
    // Data captured by the closure needs to be `Send+Sync`.
    let counter = Arc::new(AtomicUsize::new(0));
    let log = child_logger!(root, "counter" => {
        let counter = counter.clone();
        move || { counter.load(SeqCst)}
    });

    info!(log, "before-fetch-add"); // counter == 0
    counter.fetch_add(1, SeqCst);
    info!(log, "after-fetch-add"); // counter == 1

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
    trace!(log, "trace", "lazy-closure" => Box::new(move || slow_fib(40)));

    // Loggers are internally atomically reference counted so can be cloned,
    // passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {
            info!(log, "subthread", "stage" => "start");
            thread::sleep(Duration::new(1, 0));
            info!(log, "subthread", "stage" => "end");
        }
    });

    join.join().unwrap();
}

