#![feature(test)]
#![feature(conservative_impl_trait)]

#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_stream;

extern crate test;

use std::io;
use test::Bencher;
use slog::*;
use slog_stream::*;

const LONG_STRING : &'static str = "A long string that would take some time to allocate";

struct BlackBoxDrain;

impl Drain for BlackBoxDrain {
    type Error = ();
    fn log(&self, ri: &Record, o : &OwnedKeyValueList) -> std::result::Result<(), ()> {

        test::black_box((ri, o));
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

fn async_empty_json_blackbox() -> impl Drain<Error=()> {
    async_stream(BlackBoxWriter, slog_json::new().build()).ignore_err()
}

fn async_json_blackbox() -> impl Drain<Error=()> {
    async_stream(BlackBoxWriter, slog_json::default()).ignore_err()
}

fn empty_json_blackbox() -> impl Drain<Error=()> {
    stream(BlackBoxWriter, slog_json::new().build()).ignore_err()
}

fn json_blackbox() -> impl Drain<Error=()> {
    stream(BlackBoxWriter, slog_json::default()).ignore_err()
}

#[bench]
fn log_filter_out_empty(b: &mut Bencher) {
    let log = Logger::root(LevelFilter::new(BlackBoxDrain, Level::Warning), o!());

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_discard_empty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_discard_nonempty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        info!(log, ""; "what" => "write");
    });
}

#[bench]
fn log_discard_nonempty_10(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        info!(log,
              "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32, 
              "f64" => 0f64, 
              "option" => Some(0), 
              "unit" => (),
             );
    });
}


#[bench]
fn logger_clone_empty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_clone_nonempty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn logger_new_empty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        log.new(o!())
    });
}

#[bench]
fn logger_new_nonempty(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.new(o!("what" => "write"));
    });
}

#[bench]
fn logger_new_nonempty_10(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.new(o!(
                "u8" => 0u8,
                "u16" => 0u16,
                "u32" => 0u32,
                "u64" => 0u64,
                "bool" => false,
                "str" => "",
                "f32" => 0f32,
                "f64" => 0f64,
                "option" => Some(0),
                "unit" => (),
                ));
    });
}


#[bench]
fn log_discard_i32val(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        info!(log, ""; "i32" => 5);
    });
}

#[bench]
fn log_discard_i32closure(b: &mut Bencher) {
    let log = Logger::root(BlackBoxDrain, o!());

    b.iter(|| {
        info!(log, ""; "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_stream_empty_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_stream_empty_json_blackbox_i32closure(b: &mut Bencher) {

    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_stream_empty_json_blackbox_i32pushclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(5)
        }));
    });
}



#[bench]
fn log_stream_empty_json_blackbox_strclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => |_:&Record| {
            String::from(LONG_STRING)
        });
    });
}

#[bench]
fn log_stream_empty_json_blackbox_strpushclosure(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(LONG_STRING)
        }));
    });
}

#[bench]
fn log_stream_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_stream_json_blackbox_10(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_stream_empty_json_blackbox_10(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_stream_empty_json_blackbox_log_10(b: &mut Bencher) {
    let log = Logger::root(empty_json_blackbox(), o!());
    let log = log.new(o!(
            "u8" => 0u8,
            "u16" => 0u16,
            "u32" => 0u32,
            "u64" => 0u64,
            "bool" => false,
            "str" => "",
            "f32" => 0f32,
            "f64" => 0f64,
            "option" => Some(0),
            "unit" => (),
            ));

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_stream_json_blackbox_log_10(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());
    let log = log.new(o!(
            "u8" => 0u8,
            "u16" => 0u16,
            "u32" => 0u32,
            "u64" => 0u64,
            "bool" => false,
            "str" => "",
            "f32" => 0f32,
            "f64" => 0f64,
            "option" => Some(0),
            "unit" => (),
            ));

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_stream_json_blackbox_i32closure(b: &mut Bencher) {

    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => |_:&Record|{5});
    });
}

#[bench]
fn log_stream_json_blackbox_i32pushclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "i32" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(5)
        }));
    });
}

#[bench]
fn log_stream_json_blackbox_strclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => |_:&Record| {
            String::from(LONG_STRING)
        });
    });
}

#[bench]
fn log_stream_json_blackbox_strpushclosure(b: &mut Bencher) {
    let log = Logger::root(json_blackbox(), o!());

    b.iter(|| {
        info!(log, ""; "str" => PushLazy(|_:&Record, ser : ValueSerializer|{
            ser.serialize(LONG_STRING)
        }));
    });
}

#[bench]
fn log_stream_async_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_stream_async_json_blackbox_10(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_stream_async_empty_json_blackbox_i32val(b: &mut Bencher) {
    let log = Logger::root(async_empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";  "i32" => 5);
    });
}

#[bench]
fn log_stream_async_empty_json_blackbox_10(b: &mut Bencher) {
    let log = Logger::root(async_empty_json_blackbox(), o!());

    b.iter(|| {
        info!(log, "";
              "u8" => 0u8,
              "u16" => 0u16,
              "u32" => 0u32,
              "u64" => 0u64,
              "bool" => false,
              "str" => "",
              "f32" => 0f32,
              "f64" => 0f64,
              "option" => Some(0),
              "unit" => (),
              );
    });
}

#[bench]
fn log_stream_async_json_blackbox_log_10(b: &mut Bencher) {
    let log = Logger::root(async_json_blackbox(), o!());
    let log = log.new(o!(
            "u8" => 0u8,
            "u16" => 0u16,
            "u32" => 0u32,
            "u64" => 0u64,
            "bool" => false,
            "str" => "",
            "f32" => 0f32,
            "f64" => 0f64,
            "option" => Some(0),
            "unit" => (),
            ));

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn log_stream_async_empty_json_blackbox_log_10(b: &mut Bencher) {
    let log = Logger::root(async_empty_json_blackbox(), o!());
    let log = log.new(o!(
            "u8" => 0u8,
            "u16" => 0u16,
            "u32" => 0u32,
            "u64" => 0u64,
            "bool" => false,
            "str" => "",
            "f32" => 0f32,
            "f64" => 0f64,
            "option" => Some(0),
            "unit" => (),
            ));

    b.iter(|| {
        info!(log, "");
    });
}

#[bench]
fn tmp_file_write_1b(b: &mut Bencher) {
    use std::io::Write;

    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/slog-test-1b").unwrap();

    b.iter(|| {
        f.write_all(&[0]).unwrap();
    });
}


#[bench]
fn tmp_file_write_1kib(b: &mut Bencher) {
    use std::io::Write;

    let mut f = std::fs::OpenOptions::new().write(true).create(true).truncate(true).open("/tmp/slog-test-1k").unwrap();

    let buf = vec!(0u8; 1024);
    b.iter(|| {
        f.write_all(&buf).unwrap();
    });
}
