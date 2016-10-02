extern crate slog_example_lib;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let lib = slog_example_lib::MyLib::init(None);
    lib.do_the_thing();
}
