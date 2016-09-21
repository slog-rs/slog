#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;
extern crate slog_atomic;
extern crate slog_stream;

use slog::*;
use slog_atomic::*;
use slog_stream::*;
use std::thread;

use std::sync::atomic::Ordering::SeqCst;
use std::sync::atomic::AtomicUsize;
use std::sync::Arc;
use std::time::Duration;

fn slow_fib(n: u64) -> u64 {
    match n {
        0 | 1 | 2 => 1,
        n => slow_fib(n - 1) + slow_fib(n - 2),
    }
}

fn main() {
    // Create a new drain hierarchy, for the need of your program.
    // Choose from collection of existing drains, or write your own
    // `struct`-s implementing `Drain` trait.
    let drain = slog_term::streamer().async().full().build();

    // `AtomicSwitch` is a drain that wraps other drain and allows to change
    // it atomically in runtime.
    let drain = AtomicSwitch::new(drain);
    let ctrl = drain.ctrl();

    // Get a root logger that will log into a given drain.
    //
    // Note `o!` macro for more natural `OwnedKeyValue` sequence building.
    let root = Logger::root(drain.fuse(), o!("version" => env!("CARGO_PKG_VERSION"), "build-id" => "8dfljdf"));

    // Build logging context as data becomes available.
    //
    // Create child loggers from existing ones. Children clone `key: value`
    // pairs from their parents.
    let log = root.new(o!("child" => 1));

    // Closures can be used for values that change at runtime.
    // Data captured by the closure needs to be `Send+Sync`.
    let counter = Arc::new(AtomicUsize::new(0));
    let log = log.new(o!("counter" => {
        let counter = counter.clone();
        // Note the `move` to capture `counter`,
        // and unfortunate `|_ : &_|` that helps
        // current `rustc` limitations. In the future,
        // a `|_|` could work.
        move |_ : &Record| { counter.load(SeqCst)}
    }));

    // Loggers  can be cloned, passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {

            info!(log, "before-fetch-add"); // counter == 0
            counter.fetch_add(1, SeqCst);
            info!(log, "after-fetch-add"); // counter == 1

            // `AtomicSwitch` drain can swap it's interior atomically (race-free).
            ctrl.set(
                // drains are composable and reusable
                level_filter(
                    Level::Info,
                    async_stream(
                        std::io::stderr(),
                        // multiple outputs formats are supported
                        slog_json::new().build(),
                    ),
                ),
            );

            // Closures can be used for lazy evaluation:
            // This `slow_fib` won't be evaluated, as the current drain discards
            // "trace" level logging records.
            debug!(log, "debug"; "lazy-closure" => |_ : &Record| slow_fib(40));

            info!(log, "subthread"; "stage" => "start");
            thread::sleep(Duration::new(1, 0));
            info!(log, "subthread"; "stage" => "end");
        }
    });

    join.join().unwrap();
}
