//! [Bunyan](https://www.npmjs.com/package/bunyan) formatting for `slog-rs`
//!
//! ```
//! #[macro_use]
//! extern crate slog;
//! extern crate slog_bunyan;
//!
//! use slog::*;
//!
//! fn main() {
//!     let root = Logger::new_root(o!("build-id" => "8dfljdf"));
//!
//!     root.set_drain(
//!             drain::stream(
//!                 std::io::stderr(),
//!                 slog_bunyan::new()
//!                 )
//!             );
//! }
//! ```
#![warn(missing_docs)]

#[macro_use]
extern crate slog;
extern crate nix;
extern crate chrono;
extern crate slog_json;

use slog_json::Json;
use slog::logger::RecordInfo;
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

/// Create bunyan formatter
pub fn new() -> Json {
    let mut b = Json::build();
    b.set_newlines(true)
     .add_key_values(o!(
                "pid" => nix::unistd::getpid() as usize,
                "host" => get_hostname(),
                "time" => |rinfo : &RecordInfo| {
                    rinfo.ts.to_rfc3339()
                },
                "level" => |rinfo : &RecordInfo| {
                    level_to_string(rinfo.level)
                },
                // TODO: slog loggers don't have names...
                "name" => "slog-rs",
                "v" => 0usize,
                "msg" => |rinfo : &RecordInfo| {
                    rinfo.msg.clone()
                }
            ));
    b.build()
}

#[cfg(test)]
mod test {
    use super::*;
    use super::get_hostname;
    use chrono::{TimeZone, UTC};
    use nix;
    use slog::logger::RecordInfo;
    use slog::Level;
    use slog::format::Format;

    #[test]
    fn trivial() {
        let formatter = new();

        let dt = UTC.ymd(2014, 7, 8).and_hms(9, 10, 11);

        let info = RecordInfo {
            ts: dt,
            level: Level::Info,
            msg: "message".to_string(),
        };

        let mut v = vec!();
        formatter.format(&mut v, &info, &[], &[]).unwrap();

        assert_eq!(String::from_utf8_lossy(&v),
                   "{\"pid\":".to_string() + &nix::unistd::getpid().to_string() + ",\"host\":\"" +
                   &get_hostname() +
                   "\",\"time\":\"2014-07-08T09:10:11+00:00\",\"level\":30,\"name\":\"slog-rs\",\
                    \"v\":0,\"msg\":\"message\"}\n");
    }
}
