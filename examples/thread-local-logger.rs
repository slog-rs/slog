//! Example of a global thread-local loggers
//!
//! This somewhat of an anti-pattern, as loggers should be
//! context(data)-aware and trying to not pass them around is going against
//! the approach.
//!
//! `Logger`-s should be embedded in the objects so they can be accessed as
//! `self.log.info!(....)` - this way there's no need to pass as function
//! arguments.
//!
//! If the there's a need to log something about thread-context, a closure
//! should be used to return a thread-local value.  See
//! `example/thread-local-closure.rs`.
#[macro_use]
extern crate slog;

use std::cell::RefCell;
use std::thread;

use slog::Logger;

thread_local!(static TL_LOG: RefCell<Logger> = RefCell::new(
    Logger::root(slog::Discard, o!("logger" => "uninitialized"))
));

fn foo() {
    TL_LOG.with(|log| {
        info!(log.borrow(), "foo called");
    });
}

fn main() {
    let root = slog::Logger::root(
        slog::Discard,
        o!("version" => "0.0.1", "build-id" => "8dfljdf"),
        );

    TL_LOG.with(|log| {
        *log.borrow_mut() = root.new(o!("thread-id" => "main"));
    });

    let mut join = vec![];

    for i in 0..4 {
        join.push(thread::spawn({
            let root = root.clone();
            move || {

                TL_LOG.with(|log| {
                    *log.borrow_mut() = root.new(o!("thread-id" => format!("thread{}", i)));
                });

                foo();
            }
        }));
    }

    foo();

    for join in join {
        join.join().unwrap();
    }
}
