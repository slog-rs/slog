#![feature(test)]

#[macro_use]
extern crate slog;
extern crate slog_json;

extern crate test;

use std::io;
use test::Bencher;
use slog::*;
use slog::IntoLogger;

struct BlackBoxDrain;

impl Drain for BlackBoxDrain {
    fn log(&self,
           buf: &mut Vec<u8>,
           ri: &Record, o : &OwnedKeyValueNode) -> Result<()> {

        test::black_box((buf, ri, o));
        Ok(())
    }
}

struct BlackBoxWriter;

impl io::Write for BlackBoxWriter {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        test::black_box(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}

#[bench]
fn log_discard_empty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!());

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_discard_nonempty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        info!(log, "", "what" => "write");
    });
}

#[bench]
fn logger_clone_empty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!());

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_clone_nonempty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_new_empty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!());

    b.iter(|| {
        log.new(o!())
    });
}

#[bench]
fn logger_new_nonempty(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.new(o!("what" => "write"));
    });
}

#[bench]
fn log_discard_i32val(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "i32" => 5);
    });
}


#[bench]
fn log_discard_i32closure(b: &mut Bencher) {
    let log = BlackBoxDrain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_stream_json_blackbox_i32val(b: &mut Bencher) {
    let drain = stream(BlackBoxWriter, slog_json::new());

    let log = drain.into_logger(o!());

    b.iter(|| {
        info!(log, "",  "i32" => 5);
    });
}

#[bench]
fn log_stream_json_blackbox_i32closure(b: &mut Bencher) {
    let drain = stream(BlackBoxWriter, slog_json::new());

    let log = drain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_stream_json_blackbox_i32pushclosure(b: &mut Bencher) {
    let drain = stream(BlackBoxWriter, slog_json::new());

    let log = drain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "i32" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(5)
        }));
    });
}


const LONG_STRING : &'static str = "A long string that would take some time to allocate";

#[bench]
fn log_stream_json_blackbox_strclosure(b: &mut Bencher) {
    let drain = stream(BlackBoxWriter, slog_json::new());

    let log = drain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "str" => |_:&Record| {
            String::from(LONG_STRING)
        });
    });
}

#[bench]
fn log_stream_json_blackbox_strpushclosure(b: &mut Bencher) {
    let drain = stream(BlackBoxWriter, slog_json::new());

    let log = drain.into_logger(o!());

    b.iter(|| {
        info!(log, "", "str" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(LONG_STRING)
        }));
    });
}
