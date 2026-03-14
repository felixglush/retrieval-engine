use super::Filter;
use crate::error::ModelError;

#[derive(Debug, Clone, PartialEq)]
pub struct Query {
    pub query_text: Option<String>,
    pub query_embedding: Option<Vec<f32>>,
    pub namespace: String,
    pub collection: String,
    pub filters: Vec<Filter>,
    pub top_k: usize,
}

impl Query {
    pub fn validate(&self, expected_embedding_dimension: Option<usize>) -> Result<(), ModelError> {
        if self.namespace.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Query",
                field: "namespace",
            });
        }

        if self.collection.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Query",
                field: "collection",
            });
        }

        if self.top_k == 0 {
            return Err(ModelError::InvalidTopK);
        }

        let has_query_text = self
            .query_text
            .as_ref()
            .is_some_and(|text| !text.trim().is_empty());
        let has_query_embedding = self.query_embedding.is_some();

        if !has_query_text && !has_query_embedding {
            return Err(ModelError::MissingQueryInput);
        }

        if let Some(embedding) = &self.query_embedding {
            if embedding.is_empty() {
                return Err(ModelError::EmptyEmbedding {
                    type_name: "Query",
                    field: "query_embedding",
                });
            }

            if let Some(expected_dimension) = expected_embedding_dimension
                && embedding.len() != expected_dimension
            {
                return Err(ModelError::EmbeddingDimensionMismatch {
                    type_name: "Query",
                    expected: expected_dimension,
                    actual: embedding.len(),
                });
            }
        }

        for filter in &self.filters {
            filter.validate()?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Query;
    use crate::error::ModelError;
    use crate::model::{Filter, FilterOperator, MetadataValue};

    #[test]
    fn rejects_queries_without_text_or_embedding() {
        let query = Query {
            query_text: None,
            query_embedding: None,
            namespace: "workspace_abc".to_string(),
            collection: "products".to_string(),
            filters: Vec::new(),
            top_k: 10,
        };

        assert_eq!(query.validate(Some(4)), Err(ModelError::MissingQueryInput));
    }

    #[test]
    fn validates_filters_as_part_of_query_validation() {
        let query = Query {
            query_text: Some("linen dresses".to_string()),
            query_embedding: None,
            namespace: "workspace_abc".to_string(),
            collection: "products".to_string(),
            filters: vec![Filter {
                field: "price".to_string(),
                operator: FilterOperator::Lt,
                value: MetadataValue::String("cheap".to_string()),
            }],
            top_k: 10,
        };

        assert_eq!(
            query.validate(None),
            Err(ModelError::InvalidFilterValueType {
                field: "price".to_string(),
                operator: FilterOperator::Lt,
                expected: "number",
                found: "string",
            })
        );
    }
}
