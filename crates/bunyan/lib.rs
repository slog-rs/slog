//! [Bunyan](https://www.npmjs.com/package/bunyan) formatting for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_bunyan;
//! extern crate slog_stream;
//!
//! use slog::DrainExt;
//!
//! fn main() {
//!     let root = slog::Logger::root(
//!         slog_stream::stream(
//!                 std::io::stderr(),
//!                 slog_bunyan::default()
//!         ).fuse(), o!("build-id" => "8dfljdf"));
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

fn new_with_ts_fn<F>(ts_f: F) -> slog_json::FormatBuilder
    where F: Fn(&Record) -> String + Send + Sync + 'static
{
    slog_json::Format::new()
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
        ))
}

/// Create `slog_json::FormatBuilder` with bunyan key-values
pub fn new() -> slog_json::FormatBuilder {
    new_with_ts_fn(|_: &Record| chrono::Local::now().to_rfc3339())
}

/// Create `slog_json::Format` with bunyan key-values
pub fn default() -> slog_json::Format {
    new_with_ts_fn(|_: &Record| chrono::Local::now().to_rfc3339()).build()
}

#[cfg(test)]
mod test {
    use super::new_with_ts_fn;
    use super::get_hostname;
    use chrono::{TimeZone, UTC};
    use nix;
    use slog::{Record, RecordStatic};
    use slog::Level;
    use slog_stream::Format;
    use slog::OwnedKeyValueList;

    #[test]
    fn trivial() {
        let format =
            new_with_ts_fn(|_: &Record| UTC.ymd(2014, 7, 8).and_hms(9, 10, 11).to_rfc3339()).build();


        let rs = RecordStatic {
            level: Level::Info,
            file: "filepath",
            line: 11192,
            column: 0,
            function: "",
            module: "modulepath",
            target: "target"
        };

        let mut v = vec![];
        format.format(&mut v, &Record::new(&rs, format_args!("message"), &[]), &OwnedKeyValueList::root(None)).unwrap();

        assert_eq!(String::from_utf8_lossy(&v),
                   "{\"pid\":".to_string() + &nix::unistd::getpid().to_string() + ",\"host\":\"" +
                   &get_hostname() +
                   "\",\"time\":\"2014-07-08T09:10:11+00:00\",\"level\":30,\"name\":\"slog-rs\",\
                    \"v\":0,\"msg\":\"message\"}\n");
    }
}
