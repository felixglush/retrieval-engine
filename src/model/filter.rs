use super::MetadataValue;
use crate::error::ModelError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FilterOperator {
    Eq,
    Lt,
    Gt,
    Contains,
}

#[derive(Debug, Clone, PartialEq)]
pub struct Filter {
    pub field: String,
    pub operator: FilterOperator,
    pub value: MetadataValue,
}

impl Filter {
    pub fn validate(&self) -> Result<(), ModelError> {
        if self.field.trim().is_empty() {
            return Err(ModelError::EmptyField {
                type_name: "Filter",
                field: "field",
            });
        }

        let expected = match self.operator {
            FilterOperator::Eq => match &self.value {
                MetadataValue::String(_) | MetadataValue::Number(_) | MetadataValue::Boolean(_) => {
                    return Ok(());
                }
                MetadataValue::StringList(_) => "string, number, or boolean",
            },
            FilterOperator::Lt | FilterOperator::Gt => match &self.value {
                MetadataValue::Number(_) => return Ok(()),
                _ => "number",
            },
            FilterOperator::Contains => match &self.value {
                MetadataValue::String(_) => return Ok(()),
                _ => "string",
            },
        };

        Err(ModelError::InvalidFilterValueType {
            field: self.field.clone(),
            operator: self.operator,
            expected,
            found: self.value.kind(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{Filter, FilterOperator};
    use crate::error::ModelError;
    use crate::model::MetadataValue;

    #[test]
    fn accepts_numeric_less_than_filters() {
        let filter = Filter {
            field: "price".to_string(),
            operator: FilterOperator::Lt,
            value: MetadataValue::Number(200.0),
        };

        assert_eq!(filter.validate(), Ok(()));
    }

    #[test]
    fn accepts_contains_with_string_value() {
        let filter = Filter {
            field: "tags".to_string(),
            operator: FilterOperator::Contains,
            value: MetadataValue::String("linen".to_string()),
        };

        assert_eq!(filter.validate(), Ok(()));
    }

    #[test]
    fn rejects_contains_with_string_list_value() {
        let filter = Filter {
            field: "tags".to_string(),
            operator: FilterOperator::Contains,
            value: MetadataValue::StringList(vec!["linen".to_string()]),
        };

        assert_eq!(
            filter.validate(),
            Err(ModelError::InvalidFilterValueType {
                field: "tags".to_string(),
                operator: FilterOperator::Contains,
                expected: "string",
                found: "string list",
            })
        );
    }
}
