use std::collections::HashMap;

use crate::config::EngineConfig;
use crate::error::{ConfigError, StorageError};
use crate::model::{Record, RecordKey};

use super::RecordStore;

/// In-memory `RecordStore` implementation used as the first storage backend.
///
/// Records are stored in a primary key-value map, with a secondary grouping
/// that preserves insertion order within each `(namespace, collection)` scope.
#[derive(Debug, Clone)]
pub struct InMemoryRecordStore {
    config: EngineConfig,
    records: HashMap<RecordKey, Record>,
    records_by_scope: HashMap<(String, String), Vec<RecordKey>>,
}

impl InMemoryRecordStore {
    pub fn new(config: EngineConfig) -> Result<Self, ConfigError> {
        config.validate()?;

        Ok(Self {
            config,
            records: HashMap::new(),
            records_by_scope: HashMap::new(),
        })
    }

    fn scope_key(namespace: &str, collection: &str) -> (String, String) {
        (namespace.to_string(), collection.to_string())
    }
}

impl RecordStore for InMemoryRecordStore {
    fn upsert(
        &mut self,
        record: Record,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send {
        async move {
            record
                .validate(self.config.embedding_dimension)
                .map_err(StorageError::InvalidRecord)?;

            let key = record.key().map_err(StorageError::InvalidRecord)?;
            let scope_key = Self::scope_key(key.namespace(), key.collection());
            let previous_record = self.records.insert(key.clone(), record);

            let keys_for_scope = self.records_by_scope.entry(scope_key).or_default();
            if !keys_for_scope.contains(&key) {
                keys_for_scope.push(key);
            }

            Ok(previous_record)
        }
    }

    fn get(
        &self,
        key: &RecordKey,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send {
        async move { Ok(self.records.get(key).cloned()) }
    }

    fn list(
        &self,
        namespace: &str,
        collection: &str,
    ) -> impl Future<Output = Result<Vec<Record>, StorageError>> + Send {
        async move {
            let scope_key = Self::scope_key(namespace, collection);
            let Some(keys_for_scope) = self.records_by_scope.get(&scope_key) else {
                return Ok(Vec::new());
            };

            let mut records = Vec::with_capacity(keys_for_scope.len());
            for key in keys_for_scope {
                if let Some(record) = self.records.get(key) {
                    records.push(record.clone());
                }
            }

            Ok(records)
        }
    }

    fn delete(
        &mut self,
        key: &RecordKey,
    ) -> impl Future<Output = Result<Option<Record>, StorageError>> + Send {
        async move {
            let removed_record = self.records.remove(key);

            if removed_record.is_none() {
                return Ok(None);
            }

            let scope_key = Self::scope_key(key.namespace(), key.collection());
            if let Some(keys_for_scope) = self.records_by_scope.get_mut(&scope_key) {
                keys_for_scope.retain(|stored_key| stored_key != key);

                if keys_for_scope.is_empty() {
                    self.records_by_scope.remove(&scope_key);
                }
            }

            Ok(removed_record)
        }
    }
}

#[cfg(test)]
mod tests {
    use futures::executor::block_on;
    use time::{OffsetDateTime, format_description::well_known::Rfc3339};

    use super::InMemoryRecordStore;
    use crate::config::EngineConfig;
    use crate::error::{ModelError, StorageError};
    use crate::model::{MetadataMap, Record, RecordKey};
    use crate::storage::RecordStore;

    fn parse_timestamp(value: &str) -> OffsetDateTime {
        OffsetDateTime::parse(value, &Rfc3339).unwrap()
    }

    fn build_record(namespace: &str, collection: &str, id: &str, content: &str) -> Record {
        Record {
            id: id.to_string(),
            namespace: namespace.to_string(),
            collection: collection.to_string(),
            content: content.to_string(),
            embedding: Some(vec![0.1, 0.2, 0.3]),
            metadata: MetadataMap::new(),
            created_at: parse_timestamp("2026-03-14T10:00:00Z"),
            updated_at: Some(parse_timestamp("2026-03-14T11:00:00Z")),
            importance: Some(0.8),
        }
    }

    fn build_store() -> InMemoryRecordStore {
        InMemoryRecordStore::new(EngineConfig {
            embedding_dimension: Some(3),
            default_top_k: 10,
        })
        .unwrap()
    }

    #[test]
    fn insert_new_record_returns_none() {
        let mut store = build_store();
        let record = build_record("workspace_a", "products", "prod_123", "Black linen dress");

        let previous = block_on(store.upsert(record)).unwrap();

        assert_eq!(previous, None);
    }

    #[test]
    fn upsert_same_key_returns_previous_record() {
        let mut store = build_store();
        let first = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let second = build_record("workspace_a", "products", "prod_123", "Blue linen dress");

        block_on(store.upsert(first.clone())).unwrap();
        let previous = block_on(store.upsert(second.clone())).unwrap();

        assert_eq!(previous, Some(first));
        assert_eq!(
            block_on(store.get(&second.key().unwrap())).unwrap(),
            Some(second)
        );
    }

    #[test]
    fn same_id_in_different_namespaces_are_stored_separately() {
        let mut store = build_store();
        let first = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let second = build_record("workspace_b", "products", "prod_123", "Blue linen dress");

        block_on(store.upsert(first.clone())).unwrap();
        block_on(store.upsert(second.clone())).unwrap();

        assert_eq!(
            block_on(store.get(&first.key().unwrap())).unwrap(),
            Some(first)
        );
        assert_eq!(
            block_on(store.get(&second.key().unwrap())).unwrap(),
            Some(second)
        );
    }

    #[test]
    fn same_id_in_different_collections_are_stored_separately() {
        let mut store = build_store();
        let first = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let second = build_record("workspace_a", "documents", "prod_123", "Dress sizing guide");

        block_on(store.upsert(first.clone())).unwrap();
        block_on(store.upsert(second.clone())).unwrap();

        assert_eq!(
            block_on(store.get(&first.key().unwrap())).unwrap(),
            Some(first)
        );
        assert_eq!(
            block_on(store.get(&second.key().unwrap())).unwrap(),
            Some(second)
        );
    }

    #[test]
    fn invalid_record_write_returns_invalid_record_error() {
        let mut store = build_store();
        let mut record = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        record.embedding = Some(vec![0.1, 0.2]);

        let error = block_on(store.upsert(record)).unwrap_err();

        assert_eq!(
            error,
            StorageError::InvalidRecord(ModelError::EmbeddingDimensionMismatch {
                type_name: "Record",
                expected: 3,
                actual: 2,
            })
        );
    }

    #[test]
    fn get_returns_expected_record_by_composite_key() {
        let mut store = build_store();
        let record = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let key = record.key().unwrap();

        block_on(store.upsert(record.clone())).unwrap();

        assert_eq!(block_on(store.get(&key)).unwrap(), Some(record));
    }

    #[test]
    fn list_returns_only_records_for_requested_scope() {
        let mut store = build_store();
        let first = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let second = build_record("workspace_a", "products", "prod_456", "Blue linen dress");
        let other_scope = build_record("workspace_a", "documents", "doc_123", "Dress sizing guide");

        block_on(store.upsert(first.clone())).unwrap();
        block_on(store.upsert(second.clone())).unwrap();
        block_on(store.upsert(other_scope)).unwrap();

        assert_eq!(
            block_on(store.list("workspace_a", "products")).unwrap(),
            vec![first, second]
        );
    }

    #[test]
    fn delete_removes_record_from_get_and_list() {
        let mut store = build_store();
        let record = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let key = record.key().unwrap();

        block_on(store.upsert(record.clone())).unwrap();
        let removed = block_on(store.delete(&key)).unwrap();

        assert_eq!(removed, Some(record));
        assert_eq!(block_on(store.get(&key)).unwrap(), None);
        assert!(
            block_on(store.list("workspace_a", "products"))
                .unwrap()
                .is_empty()
        );
    }

    #[test]
    fn deleting_missing_key_returns_none() {
        let mut store = build_store();
        let key = RecordKey::new(
            "workspace_a".to_string(),
            "products".to_string(),
            "missing".to_string(),
        )
        .unwrap();

        assert_eq!(block_on(store.delete(&key)).unwrap(), None);
    }

    #[test]
    fn repeated_replacements_do_not_duplicate_secondary_grouping_keys() {
        let mut store = build_store();
        let first = build_record("workspace_a", "products", "prod_123", "Black linen dress");
        let second = build_record("workspace_a", "products", "prod_123", "Blue linen dress");
        let third = build_record("workspace_a", "products", "prod_123", "Green linen dress");

        block_on(store.upsert(first)).unwrap();
        block_on(store.upsert(second)).unwrap();
        block_on(store.upsert(third.clone())).unwrap();

        let scope_key = ("workspace_a".to_string(), "products".to_string());
        assert_eq!(store.records_by_scope.get(&scope_key).unwrap().len(), 1);
        assert_eq!(
            block_on(store.list("workspace_a", "products")).unwrap(),
            vec![third]
        );
    }
}
