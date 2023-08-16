
// https://platform.openai.com/docs/api-reference/authentication

use hell_core::error::{HellErrorHelper, HellResult};
use reqwest::Client;
use tracing::{info, error};

use crate::{llm::{api::LlmApi, chat::{LlmChatRequest, LlmChatSuccessResponse}, model::LlmModelList}, openai::chat::OpenaiChatRequest};

use super::{auth::ApiAuth, chat::{OpenaiChatSuccessResponse, ChatErrorResponse}, model::OpenaiModelList};



#[derive(Debug, Default)]
pub struct OpenaiApi {
    pub auth: ApiAuth,
    pub client: Client,
}

#[async_trait::async_trait]
impl LlmApi for OpenaiApi {
    /// only returns newly created assistant messages
    async fn process_chat(&self, data: LlmChatRequest) -> HellResult<LlmChatSuccessResponse> {
        let data = OpenaiChatRequest::from(data);
        info!("start processing chat with data '{:?}' ...", data);

        let response = self.client
            .post("https://api.openai.com/v1/chat/completions")
            .bearer_auth(&self.auth.api_key)
            .json(&data)
            .send().await?;

        if response.status().is_success() {
            let response: LlmChatSuccessResponse = response
                .json::<OpenaiChatSuccessResponse>()
                .await?
                .into();

            info!("chat request returned successfully '{:?}'", response);
            Ok(response)
        } else {
            let response = response
                .json::<ChatErrorResponse>()
                .await?;

            error!("chat request returned with an error '{:?}'!", response);
            Err(HellErrorHelper::request_msg_err(response.error.message))
        }
    }

    async fn querry_models(&self) -> HellResult<LlmModelList> {
        info!("start querrying models ...");

        let response = self.client
            .get("https://api.openai.com/v1/models")
            .bearer_auth(&self.auth.api_key)
            .send().await?;

        if response.status().is_success() {
            let response = response
                .json::<OpenaiModelList>().await?
                .into();

            info!("querry model request returned successfully '{:?}'", response);
            Ok(response)
        } else {
            error!("querry model request returned with an error!");
            Err(HellErrorHelper::request_msg_err("failed to query models"))
        }
    }
}
