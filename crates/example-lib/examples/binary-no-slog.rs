/// An example of an application, using a library that supports `slog` logging.
///
/// In this case the application does not provide a `Logger` (passing `None` instead)
/// and the library will use fall-back.
extern crate slog_example_lib;
extern crate env_logger;

fn main() {
    env_logger::init().unwrap();

    let lib = slog_example_lib::MyLib::init(None);
    lib.do_the_thing();
}
