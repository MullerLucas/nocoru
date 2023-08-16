use crate::llm::model::{LlmModelData, LlmModelList};

#[derive(Debug)]
pub enum OpenaiLangModel {
    Gpt35Turbo,
}

impl ToString for OpenaiLangModel {
    fn to_string(&self) -> String {
        match self {
            OpenaiLangModel::Gpt35Turbo => "gpt-3.5-turbo",
        }.to_string()
    }
}

impl From<OpenaiLangModel> for String {
    fn from(value: OpenaiLangModel) -> Self {
        value.to_string()
    }
}

impl OpenaiLangModel {
    pub fn token_limit(&self) -> u32 {
        match self {
            OpenaiLangModel::Gpt35Turbo => 4096,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenaiModelList {
    pub object: String,
    pub data: Vec<OpenaiModelData>,
}

impl From<OpenaiModelList> for LlmModelList {
    fn from(value: OpenaiModelList) -> Self {
        Self {
            models: value.data.into_iter().map(|d| d.into()).collect(),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenaiModelData {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub owned_by: String,
    pub permission: Vec<OpenaiModelPermission>,
    pub root: String,
    pub parent: Option<String>
}

impl From<OpenaiModelData> for LlmModelData {
    fn from(value: OpenaiModelData) -> Self {
        Self {
            id: value.id,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenaiModelPermission {
    pub id: String,
    pub object: String,
    pub created: u64,

    pub allow_sampling: bool,
    pub allow_create_engine: bool,
    pub allow_logprobs: bool,
    pub allow_search_indices: bool,
    pub allow_view: bool,
    pub allow_fine_tuning: bool,

    pub organization: String,
    pub group: Option<String>,
    pub is_blocking: bool,
}

