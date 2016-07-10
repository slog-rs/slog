use super::logger::RecordInfo;

use super::{BorrowedKeyValue, OwnedKeyValue};

use std::io;

/// Format record information
pub trait Format: Send + Sync + Sized {
    /// Format one logging record and write into `io`
    fn format(&self,
              io : &mut io::Write,
              info: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue])
              ;
}
