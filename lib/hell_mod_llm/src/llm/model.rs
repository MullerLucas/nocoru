// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmModelList {
    pub models: Vec<LlmModelData>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmModelData {
    pub id: String,
}

