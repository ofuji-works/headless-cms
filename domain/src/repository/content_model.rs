use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::model::content_model::ContentModel;

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct CreateContentModel {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct UpdateContentModel {
    pub id: String,
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
}

#[mockall::automock]
#[async_trait]
pub trait ContentModelRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<ContentModel>>;
    async fn create(&self, data: CreateContentModel) -> Result<ContentModel>;
    async fn update(&self, data: UpdateContentModel) -> Result<ContentModel>;
    async fn delete(&self, id: String) -> Result<()>;
}
