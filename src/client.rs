use reqwest::Client as ReqwestClient;
use serde::de::DeserializeOwned;
use serde::Serialize;

use crate::common::ApiResponse;
use crate::endpoints::completion::{CompletionRequest, CompletionsData};
use crate::endpoints::image::{ImageRequest, ImageData};
use crate::{BASE_URL, V0, V1};

pub struct Client {
    client: ReqwestClient,
    api_key: String,
}

impl Client {
    pub fn new(api_key: String) -> Self {
        Self {
            client: ReqwestClient::new(),
            api_key,
        }
    }

    pub async fn post<T, R>(&self, endpoint: impl AsRef<str>, payload: &T) -> ApiResponse<R>
    where
        T: Serialize + ?Sized,
        R: DeserializeOwned,
    {
        let url = format!("{}{}", BASE_URL, endpoint.as_ref());
        let response = self
            .client
            .post(&url)
            .header("Authorization", format!("Bearer {}", self.api_key))
            .json(payload)
            .send()
            .await?;

        Ok(response.json().await?)
    }

    pub async fn create_image(&self, request: &ImageRequest) -> ApiResponse<ImageData> {
        self.post(V0::Image, request).await
    }

    pub async fn create_completion(&self, request: &CompletionRequest) -> ApiResponse<CompletionsData> {
        self.post(V1::Completion, request).await
    }
}
