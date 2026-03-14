#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RecordKey {
    pub namespace: String,
    pub collection: String,
    pub id: String,
}

impl RecordKey {
    pub fn new(namespace: String, collection: String, id: String) -> Self {
        Self {
            namespace,
            collection,
            id,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::RecordKey;

    #[test]
    fn creates_record_keys_from_components() {
        let key = RecordKey::new(
            "workspace_abc".to_string(),
            "products".to_string(),
            "prod_123".to_string(),
        );

        assert_eq!(key.namespace, "workspace_abc");
        assert_eq!(key.collection, "products");
        assert_eq!(key.id, "prod_123");
    }
}
