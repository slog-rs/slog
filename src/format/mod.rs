use super::logger::RecordInfo;

use super::{BorrowedKeyValue, OwnedKeyValue};

use std::io;

#[allow(missing_docs)]
mod error {
    use std::io;
    use super::super::ser;

    error_chain! {
        types {
            Error, ErrorKind, ChainErr, Result;
        }
        links {
            ser::Error, ser::ErrorKind, Serialization;
        }
        foreign_links {
            io::Error, Io, "io error";
        }
        errors {}
    }
}

pub use self::error::{Error, Result, ErrorKind};

/// Format record information
pub trait Format: Send + Sync + Sized {
    /// Format one logging record and write into `io`
    fn format(&self,
              io: &mut io::Write,
              info: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue])
              -> Result<()>;
}
