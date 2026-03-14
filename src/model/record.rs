use super::{MetadataMap, RecordKey};
use crate::error::ModelError;

#[derive(Debug, Clone, PartialEq)]
pub struct Record {
    pub id: String,
    pub namespace: String,
    pub collection: String,
    pub content: String,
    pub embedding: Option<Vec<f32>>,
    pub metadata: MetadataMap,
    pub created_at: String,
    pub importance: Option<f32>,
}

impl Record {
    pub fn key(&self) -> RecordKey {
        RecordKey::new(
            self.namespace.clone(),
            self.collection.clone(),
            self.id.clone(),
        )
    }

    pub fn validate(&self, expected_embedding_dimension: Option<usize>) -> Result<(), ModelError> {
        if self.id.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Record",
                field: "id",
            });
        }

        if self.namespace.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Record",
                field: "namespace",
            });
        }

        if self.collection.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Record",
                field: "collection",
            });
        }

        if self.content.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Record",
                field: "content",
            });
        }

        if self.created_at.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Record",
                field: "created_at",
            });
        }

        if let Some(importance) = self.importance
            && !(0.0..=1.0).contains(&importance)
        {
            return Err(ModelError::ImportanceOutOfRange { importance });
        }

        if let Some(embedding) = &self.embedding {
            if embedding.is_empty() {
                return Err(ModelError::EmptyEmbedding {
                    type_name: "Record",
                    field: "embedding",
                });
            }

            if let Some(expected_dimension) = expected_embedding_dimension
                && embedding.len() != expected_dimension
            {
                return Err(ModelError::EmbeddingDimensionMismatch {
                    type_name: "Record",
                    expected: expected_dimension,
                    actual: embedding.len(),
                });
            }
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Record;
    use crate::error::ModelError;
    use crate::model::{MetadataMap, RecordKey};

    #[test]
    fn rejects_out_of_range_importance() {
        let record = Record {
            id: "prod_123".to_string(),
            namespace: "workspace_abc".to_string(),
            collection: "products".to_string(),
            content: "Black linen dress".to_string(),
            embedding: None,
            metadata: MetadataMap::new(),
            created_at: "2026-03-13T10:00:00Z".to_string(),
            importance: Some(1.5),
        };

        assert_eq!(
            record.validate(None),
            Err(ModelError::ImportanceOutOfRange { importance: 1.5 })
        );
    }

    #[test]
    fn builds_record_keys_from_records() {
        let record = Record {
            id: "prod_123".to_string(),
            namespace: "workspace_abc".to_string(),
            collection: "products".to_string(),
            content: "Black linen dress".to_string(),
            embedding: None,
            metadata: MetadataMap::new(),
            created_at: "2026-03-14T10:00:00Z".to_string(),
            importance: Some(0.8),
        };

        assert_eq!(
            record.key(),
            RecordKey::new(
                "workspace_abc".to_string(),
                "products".to_string(),
                "prod_123".to_string(),
            )
        );
    }
}
