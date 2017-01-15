<p align="center">

  <a href="https://github.com/slog-rs/slog">
  <img src="https://cdn.rawgit.com/slog-rs/misc/master/media/slog.svg" alt="slog-rs logo">
  </a>
  <br>

  <a href="https://travis-ci.org/slog-rs/slog">
      <img src="https://img.shields.io/travis/slog-rs/slog/master.svg" alt="Travis CI Build Status">
  </a>

  <a href="https://crates.io/crates/slog">
      <img src="https://img.shields.io/crates/d/slog.svg" alt="slog-rs on crates.io">
  </a>

  <a href="https://gitter.im/slog-rs/slog">
      <img src="https://img.shields.io/gitter/room/slog-rs/slog.svg" alt="slog-rs Gitter Chat">
  </a>

  <a href="https://docs.rs/releases/search?query=slog-">
      <img src="https://docs.rs/slog/badge.svg" alt="docs-rs: release versions documentation">
  </a>
  <br>
    <strong><a href="https://github.com/slog-rs/slog/wiki/Getting-started">Getting started</a></strong>
  
  <a href="//github.com/slog-rs/slog/wiki/Introduction-to-structured-logging-with-slog">Introduction</a>
  
  <a href="//github.com/slog-rs/slog/wiki/FAQ">FAQ</a>
  <br>
  <a href="https://crates.io/search?q=slog">Crate list</a>
</p>

# slog-rs - The Logging for [Rust][rust]

### Table of Contents

* [Status & news](#status--news)
* [`slog` crate](#slog-crate)
  * [Features](#features)
  * [Advantages over log crate](#advantages-over-log-crate)
  * [Terminal output example](#terminal-output-example)
  * [Using & help](#using--help)
  * [Compatibility Policy](#compatibility-policy)
* [Slog community](#slog-community)
  * [Overview](#overview)
* [Slog related resources](#slog-related-resources)

### Status & news

`slog` is an ecosystem of reusable components for structured, composable logging
for [Rust][rust].

The ambition is to be The Logging Framework for Rust. `slog` should accommodate
variety of logging features and requirements.

## `slog` crate

### Features

* flexible & easy to use
* great performance; see: [slog bench log](https://github.com/slog-rs/slog/wiki/Bench-log) and  [what makes slog fast](https://github.com/slog-rs/slog/wiki/What-makes-slog-fast)
* `#![no_std]` support (with opt-out `std` cargo feature flag)
* hierarchical loggers
* lazily evaluated values
* modular, lightweight and very extensible
	* tiny core crate that does not pull any dependencies
	* feature-crates for specific functionality
* bidirectional compatibility with standard `log` crate (`slog-stdlog` crate)
	* supports logging-scopes
	* using slog in library does not force users of the library to use slog
	  (but gives benefits); see `crates/example-lib`
* drains & output formatting
	* filtering
		* compile-time log level filter using cargo features (same as in `log` crate)
		* by level, msg, and any other meta-data
		* [`slog-envlogger`](https://github.com/slog-rs/envlogger) - port of `env_logger`
	* multiple outputs
	* efficient asynchronous logging
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
  performance waste. Use `Async` drain, and closures
  to make your logging fast.
* **run-time configuration** - `AtomicSwitch` drain allows
  changing logging behavior in the running program. You could use eg. signal
  handlers to change logging level or logging destinations. See
  [`signal` example][signal].

[signal]: https://github.com/slog-rs/misc/blob/master/examples/signal.rs
[env_logger]: https://crates.io/crates/env_logger
[functional-overview]: https://github.com/slog-rs/slog/wiki/Functional-overview

### Terminal output example

`slog-term` is only one of many `slog` features - useful showcase.

Automatic TTY detection and colors:

![slog-rs terminal full-format output](http://i.imgur.com/IUe80gU.png)

Compact vs full mode:

![slog-rs terminal compact output](http://i.imgur.com/P9u2sWP.png)
![slog-rs terminal full output](http://i.imgur.com/ENiy5H9.png)



## Using & help

See
[examples/features.rs](https://github.com/slog-rs/misc/blob/master/examples/features.rs)
for full quick code example overview.

See [faq] for answers to common questions and [wiki] for other documentation
articles. If you want to say hi, or need help use [slog-rs gitter] channel.

Read [Documentation](https://docs.rs/slog/) for details and features.

To report a bug or ask for features use [github issues][issues].

[faq]: https://github.com/slog-rs/slog/wiki/FAQ
[wiki]: https://github.com/slog-rs/slog/wiki/
[rust]: http://rust-lang.org
[slog-rs gitter]: https://gitter.im/slog-rs/slog
[issues]: //github.com/slog-rs/slog/issues

#### In your project

In Cargo.toml:

```
[dependencies]
slog = "1.2"
```

In your `main.rs`:

```
#[macro_use]
extern crate slog;
```

### Compatibility Policy

`slog` follows SemVer: this is the official policy regarding breaking changes
and minimum required versions of Rust.

Slog crates should pin minimum required version of Rust to the CI builds.
Bumping the minimum version of Rust is considered a minor breaking change,
meaning *at a minimum* the minor version will be bumped.

In order to keep from being surprised of breaking changes, it is **highly**
recommended to use the `~major.minor.patch` style in your `Cargo.toml` if you
wish to target a version of Rust that is *older* than current stable minus two
releases:

```toml
[dependencies]
slog = "~1.3.0"
```

This will cause *only* the patch version to be updated upon a `cargo update`
call, and therefore cannot break due to new features, or bumped minimum
versions of Rust.

#### Minimum Version of Rust

`slog` and it's ecosystem officially supports current stable Rust, minus
two releases, but may work with prior releases as well. For example, current
stable Rust at the time of this writing is 1.13.0, meaning `slog` is guaranteed
to compile with 1.11.0 and beyond.  At the 1.14.0 release, `slog` will be
guaranteed to compile with 1.12.0 and beyond, etc.

Upon bumping the minimum version of Rust (assuming it's within the stable-2
range), it *must* be clearly annotated in the `CHANGELOG.md`


## Slog community

### Overview

Slog related crates are hosted under [slog github
organization](https://github.com/slog-rs).

Dawid Ciężarkiewicz is the original author and current maintainer of `slog` and
therefore self-appointed benevolent dictator over the project. When working on
slog Dawid follows and expects everyone to follow his [Code of
Conduct](https://github.com/dpc/public/blob/master/COC.md).

Any particular repositories under slog ecosystem might be created, controlled,
maintained by other entities with various level of autonomy. Lets work together
toward a common goal in respectful and welcoming atmosphere!

## slog-related resources

* [24 days of Rust - structured logging](https://siciarz.net/24-days-rust-structured-logging/) - review and tutorial by Zbigniew Siciarz 2016-12-05
