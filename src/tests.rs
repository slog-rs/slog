use super::{Logger, Discard};

#[test]
fn logger_fmt_debug_sanity() {

    let root = Logger::root(Discard, o!("a" => "aa"));
    let log = root.new(o!("b" => "bb", "c" => "cc"));

    assert_eq!(format!("{:?}", log), "Logger(b, c, a)");
}
