use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::model::content::{Content, ContentStatus};

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct CreateContent {
    pub content_model_id: String,
    pub fields: Value,
    pub status: ContentStatus,
}

#[derive(Debug, Deserialize, new)]
pub struct UpdateContent {
    pub id: String,
    pub content_model_id: Option<String>,
    pub fields: Option<Value>,
    pub status: Option<ContentStatus>,
}

#[async_trait]
pub trait ContentRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<Content>>;
    async fn create(&self, data: CreateContent) -> Result<Content>;
    async fn update(&self, data: UpdateContent) -> Result<Content>;
    async fn delete(&self, id: String) -> Result<()>;
}
