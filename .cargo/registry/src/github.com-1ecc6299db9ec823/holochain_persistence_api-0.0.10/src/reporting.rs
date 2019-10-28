use super::error::{PersistenceError, PersistenceResult};
use holochain_json_api::{error::JsonError, json::JsonString};

#[derive(PartialEq, Eq, Clone, Debug, Serialize, Deserialize, DefaultJson)]
pub struct StorageReport {
    pub bytes_total: usize,
}

impl StorageReport {
    pub fn new(bytes_total: usize) -> Self {
        Self { bytes_total }
    }
}

pub trait ReportStorage {
    /// Return the number of bytes this storage implementation is using on the host system.
    /// The actual implementation is up to the author of the persistence implementation
    /// and may be disk usage or memory usage
    fn get_storage_report(&self) -> PersistenceResult<StorageReport> {
        Err(PersistenceError::ErrorGeneric(
            "Not implemented for this storage type".into(),
        ))
    }
}
