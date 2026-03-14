use crate::error::ModelError;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordKey {
    namespace: String,
    collection: String,
    id: String,
}

impl RecordKey {
    pub fn new(namespace: String, collection: String, id: String) -> Result<Self, ModelError> {
        if namespace.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "RecordKey",
                field: "namespace",
            });
        }

        if collection.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "RecordKey",
                field: "collection",
            });
        }

        if id.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "RecordKey",
                field: "id",
            });
        }

        Ok(Self {
            namespace,
            collection,
            id,
        })
    }

    pub fn namespace(&self) -> &str {
        &self.namespace
    }

    pub fn collection(&self) -> &str {
        &self.collection
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

#[cfg(test)]
mod tests {
    use super::RecordKey;
    use crate::error::ModelError;

    #[test]
    fn creates_record_keys_from_components() {
        let key = RecordKey::new(
            "workspace_abc".to_string(),
            "products".to_string(),
            "prod_123".to_string(),
        )
        .unwrap();

        assert_eq!(key.namespace(), "workspace_abc");
        assert_eq!(key.collection(), "products");
        assert_eq!(key.id(), "prod_123");
    }

    #[test]
    fn rejects_empty_key_components() {
        assert_eq!(
            RecordKey::new(
                " ".to_string(),
                "products".to_string(),
                "prod_123".to_string()
            ),
            Err(ModelError::EmptyField {
                type_name: "RecordKey",
                field: "namespace",
            })
        );
    }
}
