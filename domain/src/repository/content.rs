use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::model::content::Content;

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct CreateContent {
    pub content_model_id: String,
    pub field_values: Value,
    pub is_draft: bool,
}

#[derive(Debug, Deserialize, new)]
pub struct UpdateContent {
    pub id: String,
    pub content_model_id: String,
    pub field_values: Option<Value>,
    pub is_draft: Option<bool>,
}

#[async_trait]
pub trait ContentRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<Content>>;
    async fn create(&self, data: CreateContent) -> Result<()>;
    async fn update(&self, data: UpdateContent) -> Result<()>;
    async fn delete(&self, id: String) -> Result<()>;
}
