use std::future::Future;

use crate::error::StorageError;
use crate::model::{Record, RecordKey};

/// Defines the source-of-truth record storage operations used by the engine.
///
/// The trait returns owned `Record` values so the same interface works for
/// both in-memory backends and future database-backed implementations.
pub trait RecordStore: Send + Sync {
    /// Inserts or replaces a record and returns the previous stored value when
    /// the same composite key already existed.
    fn upsert(
        &mut self,
        record: Record,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send;

    /// Fetches a single record by its composite key.
    fn get(
        &self,
        key: &RecordKey,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send;

    /// Lists all records for one namespace and collection.
    fn list(
        &self,
        namespace: &str,
        collection: &str,
    ) -> impl Future<Output = Result<Vec<Record>, StorageError>> + Send;

    /// Removes a record by its composite key and returns the removed value
    /// when one existed.
    fn delete(
        &mut self,
        key: &RecordKey,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send;
}
