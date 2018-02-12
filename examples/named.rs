//#![feature(trace_macros)]
#[macro_use]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
use slog::*;
use slog::Drain;


fn main() {
    let decorator = slog_term::TermDecorator::new().build();
    let drain = slog_term::FullFormat::new(decorator)
        .use_original_order()
        .build()
        .fuse();
    let drain = slog_async::Async::new(drain).build().fuse();

    let log = slog::Logger::root(drain, o!("version" => "0.5"));

    //trace_macros!(true);
    info!(log, "foo is {foo}", foo = 2; "a" => "b");
    info!(log, "foo is {foo} {bar}", bar=3, foo = 2; "a" => "b");
    info!(log, "foo is {foo} {bar} {baz}", bar=3, foo = 2, baz=4; "a" => "b");
    info!(log, "foo is {foo} {bar} {baz}", bar = 3, foo = 2, baz = 4;);
    info!(log, "foo is {foo} {bar} {baz}", bar=3, foo = 2, baz=4);
    info!(log, "foo is {foo} {bar} {baz}", bar=3, foo = 2, baz=4,);
}
