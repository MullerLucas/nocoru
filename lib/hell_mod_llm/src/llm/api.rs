use hell_core::error::HellResult;

use super::{chat::{LlmChatSuccessResponse, LlmChatRequest}, model::LlmModelList};


#[async_trait::async_trait]
pub trait LlmApi: Send + Sync {
    async fn process_chat(&self, data: LlmChatRequest) -> HellResult<LlmChatSuccessResponse>;
    async fn querry_models(&self) -> HellResult<LlmModelList>;
}
