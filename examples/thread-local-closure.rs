//! Example of logger closure, returning thread-local value
#[macro_use]
extern crate slog;

use std::cell::RefCell;
use std::thread;

use slog::Logger;
use slog::Record;

thread_local!(static TL_THREAD_ID: RefCell<String> = RefCell::new("main".into()));

fn foo(log: Logger) {
    info!(log, "foo called");
}

fn main() {
    let root = slog::Logger::root(
        slog::Discard,
        o!("thread-id" => |_:&Record| {
            TL_THREAD_ID.with(|id| { id.borrow().clone() })
        }
        ),
        );

    let mut join = vec![];

    for i in 0..4 {
        join.push(thread::spawn({
            let log = root.clone();
            move || {

                TL_THREAD_ID.with(|id| {
                    *id.borrow_mut() = format!("thread{}", i);
                });

                foo(log);
            }
        }));
    }

    foo(root);

    for join in join {
        join.join().unwrap();
    }
}
