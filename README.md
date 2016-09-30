# slog-rs - Structured, composable logging for [Rust][rust]

<p align="center">
  <a href="https://travis-ci.org/dpc/slog-rs">
      <img src="https://img.shields.io/travis/dpc/slog-rs/master.svg?style=flat-square" alt="Travis CI Build Status">
  </a>
  <a href="https://crates.io/crates/slog">
      <img src="http://meritbadge.herokuapp.com/slog?style=flat-square" alt="crates.io">
  </a>
  <a href="https://gitter.im/dpc/slog-rs">
      <img src="https://img.shields.io/badge/GITTER-join%20chat-green.svg?style=flat-square" alt="Gitter Chat">
  </a>
  <br>
  <strong><a href="//dpc.github.io/slog-rs/">Documentation (master branch)</a></strong>
  <strong><a href="https://docs.rs/releases/search?query=slog">Documentation (release)</a></strong>
  <br>
  <strong><a href="//github.com/dpc/slog-rs/wiki/Introduction-to-structured-logging-with-slog">Introduction</a></strong>
  <strong><a href="//github.com/dpc/slog-rs/wiki/FAQ">FAQ</a></strong>
  <br>
  <strong><a href="https://crates.io/search?q=slog">All crates</a></strong>
</p>

### Status & news

Testing, feedback, PRs, etc. are very welcome. I'd be also very happy to share
the ownership of the project with other people to make it more community-driven.

Long term goal is to make it a go-to logging crate for Rust.

### Features

* flexible & easy to use
* great performance; see: [slog bench log](https://github.com/dpc/slog-rs/wiki/Bench-log)
* `#![no_std]` support (with opt-out `std` cargo feature flag)
* hierarchical loggers
* lazily evaluated values
* modular, lightweight and very extensible
	* tiny core create that does not pull any dependencies
	* feature-crates for specific functionality
* backward compatibility for standard `log` crate (`slog-stdlog` crate)
	* supports logging-scopes
	* using slog in library does not force users of the library to use slog
	  (but gives benefits); see `crates/example-lib`
* drains & output formatting
	* filtering
		* compile-time log level filter using cargo features (same as in `log` crate)
		* by level, msg, and any other meta-data
		* [`slog-envlogger`](https://github.com/dpc/slog-envlogger) - port of `env_logger`
	* multiple outputs
	* asynchronous IO writing
	* terminal output, with color support (`slog-term` crate)
	* Json (`slog-json` crate)
		* Bunyan (`slog-bunyan` crate)
	* syslog (`slog-syslog` crate)
	* first class custom drains

### Advantages over `log` crate

* **extensible** - `slog` provides core functionality, and some standard
  feature-set. But using traits, anyone can easily implement as
  powerful fully-custom features, publish separately and grow `slog` feature-set
  for everyone.
* **composable** - Wouldn't it be nice if you could use
  [`env_logger`][env_logger], but output authentication messages to syslog,
  while reporting errors over network in json format? With `slog` drains can
  reuse other drains! You can combine them together, chain, wrap - you name it.
* **context aware** - It's not just one global logger. Hierarchical
  loggers carry information about context of logging. When logging an error
  condition, you want to know which resource was being handled, on which
  instance of your service, using which source code build, talking with what
  peer, etc. In standard `log` you would have to repeat this information in
  every log statement. In `slog` it will happen automatically. See
  [slog-rs functional overview page][functional-overview] to understand better
  logger and drain hierarchies and log record flow through them.
* both **human and machine readable** - By keeping the key-value data format,
  meaning of logging data is preserved. Dump your logging to a JSON file, and
  send it to your data-mining system for further analysis. Don't parse it from
  lines of text anymore!
* **lazy evaluation** and **asynchronous IO** included. Waiting to
  finish writing logging information to disk, or spending time calculating
  data that will be thrown away at the current logging level, are sources of big
  performance waste. Use [`AsyncStreamer`][async-streamer] drain, and closures
  to make your logging fast.
* **run-time configuration** - [`AtomicSwitch`][atomic-switch] drain allows
  changing logging behavior in the running program. You could use eg. signal
  handlers to change logging level or logging destinations. See
  [`signal` example][signal].

[signal]: https://github.com/dpc/slog-rs/blob/master/examples/signal.rs
[env_logger]: https://crates.io/crates/env_logger
[functional-overview]: https://github.com/dpc/slog-rs/wiki/Functional-overview
[async-streamer]: http://dpc.pw/slog-rs/slog/drain/struct.AsyncStreamer.html
[atomic-switch]: http://dpc.pw/slog-rs/slog/drain/struct.AtomicSwitch.html

### Terminal output example

Colors overview:

![slog-rs terminal full-format output](http://i.imgur.com/IUe80gU.png)

Compact vs full mode:

![slog-rs terminal compact output](http://i.imgur.com/P9u2sWP.png)
![slog-rs terminal full output](http://i.imgur.com/ENiy5H9.png)



## Using & help

### Code snippet

Excerpt from `examples/features.rs`:

```rust
fn main() {
    // Create a new drain hierarchy, for the need of your program.
    // Choose from collection of existing drains, or write your own
    // `struct`-s implementing `Drain` trait.
    let drain = slog_term::streamer().async().full().build();

    // `AtomicSwitch` is a drain that wraps other drain and allows to change
    // it atomically in runtime.
    let ctrl = AtomicSwitchCtrl::new(drain);
    let drain = ctrl.drain();

    // Get a root logger that will log into a given drain.
    //
    // Note `o!` macro for more natural `OwnedKeyValue` sequence building.
    let root = Logger::root(drain.fuse(), o!("version" => VERSION, "build-id" => "8dfljdf"));

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
                        slog_json::default(),
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
```

See `examples/features.rs` for full/current code.


Read [Documentation](//dpc.github.io/slog-rs/) for details and features.

See [faq] for answers to common questions. If you want to say hi, or need help
use [#slog-rs gitter.im][slog-rs gitter].

To report a bug or ask for features use [github issues][issues].

[faq]: https://github.com/dpc/slog-rs/wiki/FAQ
[rust]: http://rust-lang.org
[slog-rs gitter]: https://gitter.im/dpc/slog-rs
[issues]: //github.com/dpc/slog-rs/issues
[log15]: //github.com/inconshreveable/log15

### Building & running

If you need to install Rust (come on, you should have done that long time ago!), use [rustup][rustup].

[rustup]: https://www.rustup.rs

#### In your project

In Cargo.toml:

```
[dependencies]
slog = "1"
```

In your `main.rs`:

```
#[macro_use]
extern crate slog;
```

### Alternatives

Please fill an issue if slog does not fill your needs. I will appreciate any
feedback. You might look into issue discussing [slog-rs
alternatives](https://github.com/dpc/slog-rs/issues/17) too.
