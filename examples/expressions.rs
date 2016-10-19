#[macro_use]
extern crate slog;
extern crate slog_term;

use slog::DrainExt;

struct Foo;

impl Foo {
    fn bar(&self) -> u32 {
        1
    }
}

struct X {
    foo : Foo,
}

fn main() {
    let log = slog::Logger::root(slog_term::streamer().full().build().fuse(), o!("version" => env!("CARGO_PKG_VERSION")));

    let foo = Foo;
    let r = X { foo: foo };

    warn!(log, "logging message");
    slog_warn!(log, "logging message");

    warn!(log, "logging message"; "a" => "b");
    slog_warn!(log, "logging message"; "a" => "b");

    warn!(log, "logging message bar={}", r.foo.bar());
    slog_warn!(log, "logging message bar={}", r.foo.bar());

    warn!(log, "logging message bar={} foo={}", r.foo.bar(), r.foo.bar());
    slog_warn!(log, "logging message bar={} foo={}", r.foo.bar(), r.foo.bar() );

    warn!(log, "logging message bar={} foo={}", r.foo.bar(), r.foo.bar(), );
    slog_warn!(log, "logging message bar={} foo={}", r.foo.bar(), r.foo.bar(), );

    warn!(log, "x" => 1; "logging message bar={}", r.foo.bar());
    slog_warn!(log, "x" => 1; "logging message bar={}", r.foo.bar());

    warn!(log, "x" => 1; "logging message bar={}", r.foo.bar(),);
    slog_warn!(log, "x" => 1; "logging message bar={}", r.foo.bar(),);

    warn!(log, "x" => 1, "y" => r.foo.bar(); "logging message bar={}", r.foo.bar());
    slog_warn!(log, "x" => 1, "y" => r.foo.bar(); "logging message bar={}", r.foo.bar());

    warn!(log, "x" => r.foo.bar(); "logging message bar={}", r.foo.bar());
    slog_warn!(log, "x" => r.foo.bar(); "logging message bar={}", r.foo.bar());

    warn!(log, "x" => r.foo.bar(), "y" => r.foo.bar(); "logging message bar={}", r.foo.bar());
    slog_warn!(log, "x" => r.foo.bar(), "y" => r.foo.bar(); "logging message bar={}", r.foo.bar());

    warn!(log, "x" => r.foo.bar(), "y" => r.foo.bar(); "logging message bar={}", r.foo.bar(),);
    slog_warn!(log, "x" => r.foo.bar(), "y" => r.foo.bar(); "logging message bar={}", r.foo.bar(),);
}
