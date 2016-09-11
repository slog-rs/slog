//! [Bunyan](https://www.npmjs.com/package/bunyan) formatting for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_bunyan;
//! extern crate slog_stream;
//!
//! use slog::Fuse;
//!
//! fn main() {
//!     let root = slog::Logger::root(
//!         slog_stream::stream(
//!                 std::io::stderr(),
//!                 slog_bunyan::new()
//!         ).fused(), o!("build-id" => "8dfljdf"));
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate nix;
extern crate chrono;
extern crate slog_json;
#[cfg(test)]
extern crate slog_stream;

use slog::Record;
use slog::Level;

fn get_hostname() -> String {

    let mut buf = vec!(0u8; 256);
    match nix::unistd::gethostname(&mut buf) {
        Ok(()) => {
            // TODO: BUG: use locale to known encoding?
            String::from_utf8_lossy(buf.split(|&b| b == 0).next().unwrap_or(&buf)).to_string()
        }
        Err(_) => "n/a".to_string(),
    }
}

fn level_to_string(level: Level) -> i8 {
    match level {
        Level::Critical => 60,
        Level::Error => 50,
        Level::Warning => 40,
        Level::Info => 30,
        Level::Debug => 20,
        Level::Trace => 10,
    }
}

fn new_with_ts_fn<F>(ts_f: F) -> slog_json::Format
    where F: Fn(&Record) -> String + Send + Sync + 'static
{
    let mut b = slog_json::Format::build();
    b.set_newlines(true)
        .add_key_values(o!(
            "pid" => nix::unistd::getpid() as usize,
            "host" => get_hostname(),
            "time" => ts_f,
            "level" => |rinfo : &Record| {
                level_to_string(rinfo.level())
            },
            // TODO: slog loggers don't have names...
            "name" => "slog-rs",
            "v" => 0usize,
            "msg" => |rinfo : &Record| {
                rinfo.msg().to_string()
            }
        ));
    b.build()
}

/// Create bunyan formatter
pub fn new() -> slog_json::Format {
    new_with_ts_fn(|_: &Record| chrono::Local::now().to_rfc3339())
}

#[cfg(test)]
mod test {
    use super::new_with_ts_fn;
    use super::get_hostname;
    use chrono::{TimeZone, UTC};
    use nix;
    use slog::Record;
    use slog::Level;
    use slog_stream::Format;
    use slog::OwnedKeyValueList;

    #[test]
    fn trivial() {
        let formatter =
            new_with_ts_fn(|_: &Record| UTC.ymd(2014, 7, 8).and_hms(9, 10, 11).to_rfc3339());


        let msg = &"message";
        let info = Record::new(Level::Info,
                               msg,
                               "filepath",
                               11192,
                               "modulepath",
                               "target",
                               &[]);

        let mut v = vec![];
        formatter.format(&mut v, &info, &OwnedKeyValueList::root(vec![])).unwrap();

        assert_eq!(String::from_utf8_lossy(&v),
                   "{\"pid\":".to_string() + &nix::unistd::getpid().to_string() + ",\"host\":\"" +
                   &get_hostname() +
                   "\",\"time\":\"2014-07-08T09:10:11+00:00\",\"level\":30,\"name\":\"slog-rs\",\
                    \"v\":0,\"msg\":\"message\"}\n");
    }
}
