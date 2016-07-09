use super::logger::RecordInfo;

use super::{BorrowedKeyValue, OwnedKeyValue};

/// Format record information
pub trait Format: Send + Sync + Sized {
    /// Format one logging record into `String`
    fn format(&self,
              info: &RecordInfo,
              logger_values: &[OwnedKeyValue],
              record_values: &[BorrowedKeyValue])
              -> String;
}
