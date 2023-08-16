use super::role::LlmChatRole;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct LlmChatMsg {
    pub role: LlmChatRole,
    pub content: String,
}

impl LlmChatMsg {
    pub fn new(role: LlmChatRole, content: impl Into<String>) -> Self {
        Self {
            role,
            content: content.into(),
        }
    }

    pub fn new_system(content: impl Into<String>) -> Self {
        Self::new(LlmChatRole::System, content)
    }

    pub fn new_assistant(content: impl Into<String>) -> Self {
        Self::new(LlmChatRole::Assistant, content)
    }

    pub fn new_user(content: impl Into<String>) -> Self {
        Self::new(LlmChatRole::User, content)
    }
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmChatSuccessResponse {
    pub messages: Vec<LlmChatMsg>,
}

// ----------------------------------------------------------------------------

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub struct LlmChatRequest {
    pub model: String,
    pub messages: Vec<LlmChatMsg>,
    pub temperature: f32,
}

impl LlmChatRequest {
    pub fn new(model: impl Into<String>, messages: Vec<LlmChatMsg>, temperature: f32) -> Self {
        Self {
            model: model.into(),
            messages,
            temperature,
        }
    }
}

// ----------------------------------------------------------------------------
