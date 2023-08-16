use hell_core::error::{HellErrorHelper, HellResult};

use crate::llm::{role::LlmChatRole, chat::{LlmChatMsg, LlmChatRequest, LlmChatSuccessResponse}};
use super::{model::OpenaiLangModel, role::OpenaiChatRole};


// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenaiChatMessage {
    pub role: OpenaiChatRole,
    pub content: String,
}

impl From<OpenaiChatMessage> for LlmChatMsg {
    fn from(value: OpenaiChatMessage) -> Self {
        Self {
            role: LlmChatRole::from(value.role),
            content: value.content,
        }
    }
}

impl From<LlmChatMsg> for OpenaiChatMessage {
    fn from(value: LlmChatMsg) -> Self {
        Self {
            role: OpenaiChatRole::from(value.role),
            content: value.content,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct OpenaiChatRequest {
    pub model: String,
    pub messages: Vec<OpenaiChatMessage>,
    pub max_tokens: Option<u32>,
    pub temperature: f32,
}

impl OpenaiChatRequest {
    pub fn new(model: OpenaiLangModel, messages: Vec<OpenaiChatMessage>, max_tokens: Option<u32>, temperature: f32) -> HellResult<Self> {
        if let Some(max_tokens) = max_tokens {
            if max_tokens > model.token_limit() {
                return Err(HellErrorHelper::request_msg_err("Max tokens must be less than or equal to the model's token limit"));
            }
        }

        Ok(Self {
            model: model.to_string(),
            messages,
            max_tokens,
            temperature,
        })
    }
}

impl From<LlmChatRequest> for OpenaiChatRequest {
    fn from(value: LlmChatRequest) -> Self {
        Self {
            model: value.model,
            messages: value.messages.into_iter().map(|msg| msg.into()).collect(),
            max_tokens: None,
            temperature: value.temperature,
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatUsageInfo {
    pub prompt_tokens: u32,
    pub completion_tokens: u32,
    pub total_tokens: u32,
}

// ----------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
pub struct ChatChoice {
    pub message: OpenaiChatMessage,
    /// ## Possible Values:
    /// - **stop**: API returned complete model output
    /// - **length**: Incomplete model output due to *max_tokens* parameter or token limit
    /// - **content_filter**: Omitted content due to a flag from our content filters
    /// - **null**: API response still in progress or incomplete
    pub finish_reason: String,
    pub index: u64
}

// ----------------------------------------------------------------------------

#[derive(serde::Serialize, serde::Deserialize)]
pub struct OpenaiChatSuccessResponse {
    pub id: String,
    pub object: String,
    pub created: u64,
    pub model: String,
    pub usage: ChatUsageInfo,
    pub choices: Vec<ChatChoice>
}

impl From<OpenaiChatSuccessResponse> for LlmChatSuccessResponse {
    fn from(value: OpenaiChatSuccessResponse) -> Self {
        Self {
            messages: value.choices.into_iter().map(|choice| choice.message.into()).collect(),
        }
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatError {
    pub message: String,
    #[serde(rename = "type")]
    pub error_type: String,
    pub param: Option<String>,

    /// ## API errors
    /// **401** - Invalid Authentication
    /// **401** - Incorrect API key provided
    /// **401** - You must be a member of an organization to use the API
    /// **429** - Rate limit reached for requests
    /// **429** - You exceeded your current quota, please check your plan and billing details
    /// **429** - The engine is currently overloaded, please try again later
    /// **500** - The server had an error while processing your request
    pub code: Option<String>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct ChatErrorResponse {
    pub error: ChatError
}
