# slog-rs - Structured, composable logging for Rust

<p align="center">
  <a href="https://travis-ci.org/dpc/slog-rs">
      <img src="https://img.shields.io/travis/dpc/slog-rs/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://crates.io/crates/slog">
      <img src="http://meritbadge.herokuapp.com/slog?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/dpc">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/slog-rs/">Documentation</a></strong>
</p>


## Code snippet

``` rust
fn main() {
    // Create a new group of loggers, sharing one drain.
    //
    // Note `o!` macro for more natural `OwnedKeyValue` sequence building.
    let root = Logger::new_root(o!("version" => VERSION, "build-id" => "8dfljdf"));

    // Set drains to specify the output format and destination.
    root.set_drain(slog_term::async_stderr());

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
        move |_ : &_| { counter.load(SeqCst)}
    }));

    // Note `b!` macro for more natural `BorrowedKeyValue` sequence building.
    log.info("before-fetch-add", b!()); // counter == 0
    counter.fetch_add(1, SeqCst);
    log.info("after-fetch-add", b!()); // counter == 1

    // Drains can be swapped atomically (race-free).
    log.set_drain(
        // drains are composable
        drain::filter_level(
            Level::Info,
            drain::stream(
                drain::async(std::io::stderr()),
                // multiple outputs formats are supported
                slog_json::new(),
                ),
            ),
        );

    // Closures can be used for lazy evaluation:
    // This `slow_fib` won't be evaluated, as the current drain discards
    // "trace" level logging records.
    log.debug("debug", b!("lazy-closure" => |_ : &_| slow_fib(40)));

    // Loggers are internally atomically reference counted so can be cloned,
    // passed between threads and stored without hassle.
    let join = thread::spawn({
        let log = log.clone();
        move || {
            log.info("subthread", b!("stage" => "start"));
            thread::sleep(Duration::new(1, 0));
            log.info("subthread", b!("stage" => "end"));
        }
    });

    join.join().unwrap();
}
```

See `examples/features.rs` for full code.

## Introduction

Structured, composable logging for [Rust][rust]. Work in progress, but usable
already.

Heavily inspired by [log15] for Go, which I liked so much, that I want it in
Rust too.

Read [Documentation](//dpc.github.io/slog-rs/) for details and features.

If you want to say hi, or need help use [#dpc gitter.im][dpc gitter].

To report a bug or ask for features use [github issues][issues].

[rust]: http://rust-lang.org
[dpc gitter]: https://gitter.im/dpc/dpc
[issues]: //github.com/dpc/slog-rs/issues
[log15]: //github.com/inconshreveable/log15

## Features

* easy to use
* hierarchal loggers
* lazily evaluated values
* extensible: first-class custom implementations
* drains & output formatting
	* filtering
	* multiple outputs
	* asynchronous IO writing
	* terminal output, with color support (`slog-term` crate)
	* Json (`slog-json` crate)
		* Bunyan (`slog-bunyan` crate)
	* more to come (syslog, network)

## Building & running

If you need to install Rust (come on, you should have done that long time ago!), use [rustup][rustup].

[rustup]: https://rustup.rs

### In your project

In Cargo.toml:

```
[dependencies]
slog = "*"
```

In your `main.rs`:

```
#[macro_use]
extern crate slog;
```
