pub use crate::error::ConfigError;

#[derive(Debug, Clone, PartialEq)]
pub struct EngineConfig {
    pub embedding_dimension: Option<usize>,
    pub default_top_k: usize,
}

impl EngineConfig {
    pub fn validate(&self) -> Result<(), ConfigError> {
        if self.default_top_k == 0 {
            return Err(ConfigError::InvalidDefaultTopK);
        }

        if let Some(embedding_dimension) = self.embedding_dimension
            && embedding_dimension == 0
        {
            return Err(ConfigError::InvalidEmbeddingDimension);
        }

        Ok(())
    }
}

impl Default for EngineConfig {
    fn default() -> Self {
        Self {
            embedding_dimension: None,
            default_top_k: 10,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::EngineConfig;
    use crate::error::ConfigError;

    #[test]
    fn default_config_is_valid() {
        let config = EngineConfig::default();

        assert_eq!(config.validate(), Ok(()));
    }

    #[test]
    fn rejects_zero_default_top_k() {
        let config = EngineConfig {
            embedding_dimension: Some(384),
            default_top_k: 0,
        };

        assert_eq!(config.validate(), Err(ConfigError::InvalidDefaultTopK));
    }
}
