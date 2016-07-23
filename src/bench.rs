use test::Bencher;
use super::*;
use super::drain::IntoLogger;

#[bench]
fn empty_log_discard(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!());

    b.iter(|| {
        log.info("", b!());
    });
}

#[bench]
fn nonempty_log_discard(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.info("", b!("what" => "write"));
    });
}

#[bench]
fn empty_logger_clone(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!());

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn nonempty_logger_clone(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.clone()
    });
}

#[bench]
fn empty_logger_new(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!());

    b.iter(|| {
        log.new(o!())
    });
}


#[bench]
fn nonempty_logger_new(b: &mut Bencher) {
    let d = drain::discard();
    let log = d.into_logger(o!("build" => "123456", "id" => 123456));

    b.iter(|| {
        log.new(o!("what" => "write"));
    });
}
