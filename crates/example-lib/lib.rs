//! Example of a library utilizing `slog`
#![warn(missing_docs)]

/// Re-export slog
///
/// Users of this library can, but don't have to use slog to build their own
/// loggers
#[macro_use]
pub extern crate slog ;
extern crate slog_stdlog;

use slog::DrainExt;

/// MyLib main struct
pub struct MyLib {
    logger : slog::Logger,
}

impl MyLib {

    /// Initialize `MyLib`, possibly providing custom logger
    ///
    /// `logger = None`, will make `MyLib` log to the standard `log`
    /// crate.
    pub fn init(logger : Option<slog::Logger>) -> Self {
        MyLib {
            logger: logger.unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// Do something
    pub fn do_the_thing(&self) {
        debug!(self.logger, "starting"; "what" => "the_thing");
    }
}
