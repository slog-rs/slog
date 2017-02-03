#[cfg(test)]
mod tests {

    use *;
    use {Drain};

    /// ensure o! macro expands without error inside a module
    #[test]
    fn test_o_macro_expansion() {
        let _ = Logger::root(Discard, o!("a" => "aa"));
    }

    #[test]
    fn logger_fmt_debug_sanity() {

        let root = Logger::root(Discard, o!("a" => "aa"));
        let log = root.new(o!("b" => "bb", "c" => "cc"));

        assert_eq!(format!("{:?}", log), "Logger(c, b, a)");
    }

    #[test]
    fn multichain() {
        struct CheckOwned;

        impl Drain for CheckOwned{
            type Error = Never;
            fn log(&self, record: &Record, values : &OwnedKeyValueList) -> result::Result<(), Self::Error> {
                assert_eq!(format!("{}", record.msg()), format!("{:?}", values));
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
