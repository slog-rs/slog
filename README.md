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
    mioco::start(|| -> io::Result<()> {
        let addr = listend_addr();

        let listener = try!(TcpListener::bind(&addr));

        println!("Starting tcp echo server on {:?}", try!(listener.local_addr()));

        loop {
            let mut conn = try!(listener.accept());

            mioco::spawn(move || -> io::Result<()> {
                let mut buf = [0u8; 1024 * 16];
                loop {
                    let size = try!(conn.read(&mut buf));
                    if size == 0 {/* eof */ break; }
                    let _ = try!(conn.write_all(&mut buf[0..size]));
                }

                Ok(())
            });
        }
    }).unwrap().unwrap();
```
## Introduction

Structured, composable logging for [Rust][rust]

Read [Documentation](//dpc.github.io/slog/) for details and features.

If you want to say hi, or need help use [#dpc gitter.im][dpc gitter].

To report a bug or ask for features use [github issues][issues].

[rust]: http://rust-lang.org
[dpc gitter]: https://gitter.im/dpc/dpc
[issues]: //github.com/dpc/mioco/issues

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
