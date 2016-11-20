//! Example of a library utilizing `slog` logging under the-hood but providing it's user with
//! possibility to ignore `slog` functionality.
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
    /// `logger = None`, will make `MyLib` log to the `slog-stdlog`
    /// drain. This make the library effectively work the same
    /// as it was just using `log` instead of `slog`.
    ///
    /// `Into` trick allows passing `Logger` directly, without the `Some` part.
    /// See http://xion.io/post/code/rust-optional-args.html
    pub fn init<L : Into<Option<slog::Logger>>>(logger : L) -> Self {
        MyLib {
            logger: logger.into().unwrap_or(slog::Logger::root(slog_stdlog::StdLog.fuse(), o!())),
        }
    }

    /// Do something
    pub fn do_the_thing(&self) {
        debug!(self.logger, "starting"; "what" => "the_thing");
    }
}
