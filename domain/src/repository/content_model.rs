use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;

use crate::model::{content_model::ContentModel, field_meta::FieldMeta};

#[derive(Debug, new)]
pub struct CreateContentModel {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
    pub fields: Vec<FieldMeta>,
}

#[async_trait]
pub trait ContentModelRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<ContentModel>>;
    async fn create(&self, content_model: CreateContentModel) -> Result<()>;
    // async fn update(&self) -> Result<()>;
    async fn delete(&self, id: String) -> Result<()>;
}
