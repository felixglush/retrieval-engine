use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq)]
pub enum MetadataValue {
    String(String),
    Number(f64),
    Boolean(bool),
    StringList(Vec<String>),
}

pub type MetadataMap = HashMap<String, MetadataValue>;

impl MetadataValue {
    pub fn kind(&self) -> &'static str {
        match self {
            Self::String(_) => "string",
            Self::Number(_) => "number",
            Self::Boolean(_) => "boolean",
            Self::StringList(_) => "string list",
        }
    }
}
