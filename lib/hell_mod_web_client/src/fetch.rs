use hell_core::error::HellResult;
use wasm_bindgen::{JsCast, JsValue};
use web_sys::{RequestInit, RequestMode, Request, Response, Window};
use wasm_bindgen_futures::JsFuture;
use crate::error::ErrToWebHellErr;




#[derive(Debug)]
pub struct FetchAsync {
    window: Window,
    prefix: String,
}

impl FetchAsync {
    pub fn new(window: Window, prefix: impl Into<String>) -> Self {
        Self {
            window,
            prefix: prefix.into(),
        }
    }
}

impl FetchAsync {
    #[inline]
    pub fn qualify_url(&self, url: &str) -> String {
        format!("{}/{}", &self.prefix, url)
    }

    pub async fn fetch_json<'de, T>(&self, url: &str, mut opts: RequestInit) -> HellResult<T>
    where T: serde::de::DeserializeOwned
    {
        let url = self.qualify_url(url);
        opts.mode(RequestMode::Cors);

        let request = Request::new_with_str_and_init(&url, &opts).to_web_hell_err()?;
        request.headers().set("Accept", "application/json").to_web_hell_err()?;
        request.headers().set("Content-Type", "application/json").to_web_hell_err()?;

        let resp_value = JsFuture::from(self.window.fetch_with_request(&request)).await.to_web_hell_err()?;
        assert!(resp_value.is_instance_of::<Response>());
        let resp: Response = resp_value.dyn_into().to_web_hell_err()?;
        let json = JsFuture::from(resp.json().to_web_hell_err()?).await.to_web_hell_err()?;

        Ok(serde_wasm_bindgen::from_value(json)?)
    }

    #[inline]
    pub async fn fetch_json_with_body<'de, B, R>(&self, url: &str, body: &B, mut opts: RequestInit) -> HellResult<R>
    where B: serde::Serialize,
          R: serde::de::DeserializeOwned
    {
        let body = serde_json::to_string(body)?;
        let body = JsValue::from_str(&body);
        opts.body(Some(&body));
        self.fetch_json(url, opts).await
    }

    pub async fn get<'de, T>(&self, url: &str) -> HellResult<T>
    where T: serde::de::DeserializeOwned
    {
        let mut opts = RequestInit::new();
        opts.method("GET");
        self.fetch_json(url, opts).await
    }

    pub async fn post<'de, B, R>(&self, url: &str, body: &B) -> HellResult<R>
    where B: serde::Serialize,
          R: serde::de::DeserializeOwned
    {
        let mut opts = RequestInit::new();
        opts.method("POST");
        self.fetch_json_with_body(url, body, opts).await
    }

    pub async fn put<'de, B, R>(&self, url: &str, body: &B) -> HellResult<R>
    where B: serde::Serialize,
          R: serde::de::DeserializeOwned
    {
        let mut opts = RequestInit::new();
        opts.method("PUT");
        self.fetch_json_with_body(url, body, opts).await
    }

    pub async fn delete<'de, T>(&self, url: &str) -> HellResult<T>
    where T: serde::de::DeserializeOwned
    {
        let mut opts = RequestInit::new();
        opts.method("DELETE");
        self.fetch_json(url, opts).await
    }
}

