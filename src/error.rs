use crate::model::FilterOperator;
use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigError {
    InvalidDefaultTopK,
    InvalidEmbeddingDimension,
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidDefaultTopK => {
                write!(f, "config.default_top_k must be greater than zero")
            }
            Self::InvalidEmbeddingDimension => {
                write!(
                    f,
                    "config.embedding_dimension must be greater than zero when provided"
                )
            }
        }
    }
}

impl std::error::Error for ConfigError {}

#[derive(Debug, Clone, PartialEq)]
pub enum StorageError {
    InvalidRecord(ModelError),
    Backend(String),
}

impl fmt::Display for StorageError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidRecord(error) => write!(f, "invalid record: {error}"),
            Self::Backend(message) => write!(f, "storage backend error: {message}"),
        }
    }
}

impl std::error::Error for StorageError {}

#[derive(Debug, Clone, PartialEq)]
pub enum ModelError {
    EmptyField {
        type_name: &'static str,
        field: &'static str,
    },
    EmptyEmbedding {
        type_name: &'static str,
        field: &'static str,
    },
    EmbeddingDimensionMismatch {
        type_name: &'static str,
        expected: usize,
        actual: usize,
    },
    InvalidFilterValueType {
        field: String,
        operator: FilterOperator,
        expected: &'static str,
        found: &'static str,
    },
    InvalidTopK,
    MissingQueryInput,
    ImportanceOutOfRange {
        importance: f32,
    },
    TimestampOutOfOrder {
        type_name: &'static str,
        earlier_field: &'static str,
        later_field: &'static str,
    },
}

impl fmt::Display for ModelError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyField { type_name, field } => {
                write!(f, "{type_name}.{field} must not be empty")
            }
            Self::EmptyEmbedding { type_name, field } => {
                write!(f, "{type_name}.{field} must not be empty when provided")
            }
            Self::EmbeddingDimensionMismatch {
                type_name,
                expected,
                actual,
            } => write!(
                f,
                "{type_name} embedding dimension mismatch: expected {expected}, got {actual}"
            ),
            Self::InvalidFilterValueType {
                field,
                operator,
                expected,
                found,
            } => write!(
                f,
                "filter '{field}' uses operator {operator:?}, which expects {expected}, found {found}"
            ),
            Self::InvalidTopK => write!(f, "query.top_k must be greater than zero"),
            Self::MissingQueryInput => {
                write!(f, "query must include query_text, query_embedding, or both")
            }
            Self::ImportanceOutOfRange { importance } => {
                write!(
                    f,
                    "record.importance must be between 0.0 and 1.0, got {importance}"
                )
            }
            Self::TimestampOutOfOrder {
                type_name,
                earlier_field,
                later_field,
            } => write!(
                f,
                "{type_name}.{later_field} must be greater than or equal to {type_name}.{earlier_field}"
            ),
        }
    }
}

impl std::error::Error for ModelError {}
