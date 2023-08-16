#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize)]
pub enum LlmChatRole {
    System,
    Assistant,
    User,
}

impl ToString for LlmChatRole {
    fn to_string(&self) -> String {
        match self {
            LlmChatRole::System => "system",
            LlmChatRole::Assistant => "assistant",
            LlmChatRole::User => "user",
        }.to_string()
    }
}

// ----------------------------------------------------------------------------
