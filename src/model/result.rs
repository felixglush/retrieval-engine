use super::MetadataMap;

#[derive(Debug, Clone, PartialEq)]
pub struct SearchResult {
    pub record_id: String,
    pub namespace: String,
    pub collection: String,
    pub content_preview: String,
    pub metadata: MetadataMap,
    pub final_score: f32,
    pub lexical_score: Option<f32>,
    pub semantic_score: Option<f32>,
    pub freshness_score: Option<f32>,
    pub importance_score: Option<f32>,
}
