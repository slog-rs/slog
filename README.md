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
* [Slog community](#slog-community)
* [Slog related resources](#slog-related-resources)

### Introduction

`slog` is an ecosystem of reusable components for structured, extensible,
composable logging for [Rust][rust].

The ambition is to be The Logging Framework for Rust. `slog` should accommodate a
variety of logging features and requirements.

### Status & news

`slog v2` (versions `2.x.y`) is the latest stable major release. API of `slog
v2` is very similar to `slog v1` and most changes are under the hood, unleashing
more power. Porting should be relatively easy.

See [`slog v2` wiki page](https://github.com/slog-rs/slog/wiki/slog-v2)
for more information.

Note: at the time of writing, many important `slog` ecosystem crates might
still not have a stable release for `slog v2` and you might need to point
to their pre-release versions. Eg. (`Cargo.toml` excerpt):

```
slog-term = "~2.0.0-4"
```

`slog v1` (versions `1.x.y`) is a previous stable major release. No new development
is planned, but bugfixes and PRs are always welcome.

### Features & technical documentation

Most of the interesting documentation is using rustdoc itself.

You can view it at [docs.rs/slog for v1](https://docs.rs/slog/1/) or [docs.rs/slog
for v2](https://docs.rs/slog/2/).

### Terminal output example

`slog-term` is only one of many `slog` features - useful showcase,
featuring eg. automatic TTY detection and colors.

See following screenshot: same output in both compact and full output mode.

![slog-rs terminal example output](http://i.imgur.com/mqrG8yL.png)


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
slog = "2"
```

In your `main.rs`:

```
#[macro_use]
extern crate slog;
```

## Slog community

Slog related crates are hosted under [slog github
organization](https://github.com/slog-rs).

Dawid Ciężarkiewicz is the original author and current maintainer of `slog` and
therefore self-appointed benevolent dictator over the project. When working on
slog Dawid follows and expects everyone to follow his [Code of
Conduct](https://github.com/dpc/public/blob/master/COC.md).

Any particular repositories under slog ecosystem might be created, controlled,
maintained by other entities with various levels of autonomy. Lets work together
toward a common goal in a respectful and welcoming atmosphere!

## slog-related resources

* [24 days of Rust - structured logging](https://siciarz.net/24-days-rust-structured-logging/) - review and tutorial by Zbigniew Siciarz 2016-12-05
* [Writing GStreamer Elements in Rust (Part 4): Logging, COWs and Plugins](https://coaxion.net/blog/2017/03/writing-gstreamer-elements-in-rust-part-4-logging-cows-and-plugins/) - experiences using Rust & slog for GStreamer
