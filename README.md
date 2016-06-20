# slog-rs

<p align="center">
  <a href="https://travis-ci.org/dpc/slog-rs">
      <img src="https://img.shields.io/travis/dpc/mioco/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://ci.appveyor.com/project/dpc/slog-rs/branch/master">
      <img src="https://ci.appveyor.com/api/projects/status/p5rjfbqw2a3pxc4o/branch/master?svg=true" alt="App Veyor Build Status">
  </a>
  <a href="https://crates.io/crates/slog">
      <img src="http://meritbadge.herokuapp.com/slog?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/slog">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/slog/">Documentation</a></strong>
</p>


## Code snippet

``` rust
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
extern crate slog;
```
