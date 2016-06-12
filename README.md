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
```
## Introduction

Structured, composable logging for [Rust][rust]

Inspired by [log15] for Go. Work in progress.

Read [Documentation](//dpc.github.io/slog-rs/) for details and features.

If you want to say hi, or need help use [#dpc gitter.im][dpc gitter].

To report a bug or ask for features use [github issues][issues].

[rust]: http://rust-lang.org
[dpc gitter]: https://gitter.im/dpc/dpc
[issues]: //github.com/dpc/slog-rs/issues
[log15]: //github.com/inconshreveable/log15

## Building & running

If you need to install Rust, use [rustup][rustup].

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
