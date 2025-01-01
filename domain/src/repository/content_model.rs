use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

use crate::model::content_model::ContentModel;

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct CreateContentModel {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
    pub fields: Value,
}

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct UpdateContentModel {
    pub id: String,
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
    pub fields: Option<Value>,
}

#[async_trait]
pub trait ContentModelRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<ContentModel>>;
    async fn create(&self, data: CreateContentModel) -> Result<()>;
    async fn update(&self, data: UpdateContentModel) -> Result<()>;
    async fn delete(&self, id: String) -> Result<()>;
}
