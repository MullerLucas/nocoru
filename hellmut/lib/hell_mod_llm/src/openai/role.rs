use crate::llm::role::LlmChatRole;

#[derive(Debug, serde::Serialize, serde::Deserialize)]
pub enum OpenaiChatRole {
    #[serde(rename = "system")]
    System,
    #[serde(rename = "assistant")]
    Assistant,
    #[serde(rename = "user")]
    User,
}

impl ToString for OpenaiChatRole {
    fn to_string(&self) -> String {
        match self {
            OpenaiChatRole::System => "system",
            OpenaiChatRole::Assistant => "assistant",
            OpenaiChatRole::User => "user",
        }.to_string()
    }
}

impl From<OpenaiChatRole> for LlmChatRole {
    fn from(value: OpenaiChatRole) -> Self {
        match value {
            OpenaiChatRole::System    => LlmChatRole::System,
            OpenaiChatRole::Assistant => LlmChatRole::Assistant,
            OpenaiChatRole::User      => LlmChatRole::User,
        }
    }
}

impl From<LlmChatRole> for OpenaiChatRole {
    fn from(value: LlmChatRole) -> Self {
        match value {
            LlmChatRole::System    => OpenaiChatRole::System,
            LlmChatRole::Assistant => OpenaiChatRole::Assistant,
            LlmChatRole::User      => OpenaiChatRole::User,
        }
    }
}
