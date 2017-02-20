use {Logger, Discard};

// Separate module to test lack of imports
mod no_imports {
    use {Logger, Discard};
    /// ensure o! macro expands without error inside a module
    #[test]
    fn test_o_macro_expansion() {
        let _ = Logger::root(Discard, o!("a" => "aa"));
    }
    /// ensure o! macro expands without error inside a module
    #[test]
    fn test_slog_o_macro_expansion() {
        let _ = Logger::root(Discard, slog_o!("a" => "aa"));
    }
}

#[cfg(feature = "std")]
mod std_only {
    use super::super::*;

    use std;

    #[test]
    fn logger_fmt_debug_sanity() {

        let root = Logger::root(Discard, o!("a" => "aa"));
        let log = root.new(o!("b" => "bb", "c" => "cc"));

        assert_eq!(format!("{:?}", log), "Logger(c, b, a)");
    }


    #[test]
    fn multichain() {
        #[derive(Clone)]
        struct CheckOwned;

        impl Drain for CheckOwned {
            type Ok = ();
            type Err = Never;
            fn log(&self,
                   record: &Record,
                   values: &OwnedKVList)
                   -> std::result::Result<Self::Ok, Self::Err> {
                assert_eq!(format!("{}", record.msg()),
                           format!("{:?}", values));
                Ok(())
            }
        }

        let root = Logger::root(CheckOwned, o!("a" => "aa"));
        let log = root.new(o!("b1" => "bb", "b2" => "bb"));

        info!(log, "(b2, b1, a)");

        let log = Logger::root(log, o!("c" => "cc"));
        info!(log, "(c, b2, b1, a)");
        let log = Logger::root(log, o!("d1" => "dd", "d2" => "dd"));
        info!(log, "(d2, d1, c, b2, b1, a)");
    }
}

#[test]
fn expressions() {

    struct Foo;

    impl Foo {
        fn bar(&self) -> u32 {
            1
        }
    }

    struct X {
        foo: Foo,
    }


    let log = Logger::root(Discard, o!("version" => env!("CARGO_PKG_VERSION")));

    let foo = Foo;
    let r = X { foo: foo };

    warn!(log, "logging message");
    slog_warn!(log, "logging message");

    info!(log, #"with tag", "logging message");
    slog_info!(log, #"with tag", "logging message");

    warn!(log, "logging message"; "a" => "b");
    slog_warn!(log, "logging message"; "a" => "b");

    warn!(log, "logging message bar={}", r.foo.bar());
    slog_warn!(log, "logging message bar={}", r.foo.bar());

    warn!(log,
          "logging message bar={} foo={}",
          r.foo.bar(),
          r.foo.bar());
    slog_warn!(log,
               "logging message bar={} foo={}",
               r.foo.bar(),
               r.foo.bar());

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


#[test]
fn makers() {
    use *;
    let drain =
        Duplicate(Discard.filter(|r| r.level().is_at_least(Level::Info)),
                  Discard.filter_level(Level::Warning))
            .map(Fuse);
    let _log = Logger::root(Arc::new(drain),
                            o!("version" => env!("CARGO_PKG_VERSION")));
}

#[test]
fn logger_to_arc() {
    use *;

    fn takes_arced_drain(_l: Logger) {}

    let drain =
        Duplicate(Discard.filter(|r| r.level().is_at_least(Level::Info)),
                  Discard.filter_level(Level::Warning))
            .map(Fuse);
    let log = Logger::root(Arc::new(drain),
                           o!("version" => env!("CARGO_PKG_VERSION")));

    takes_arced_drain(log.to_arc());
}
