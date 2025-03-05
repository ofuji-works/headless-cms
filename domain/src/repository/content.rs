use crate::model::content::{Content, ContentStatus};

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CreateContent {
    pub category_id: String,
    pub fields: serde_json::Value,
    pub tag_ids: Vec<String>,
    pub status: ContentStatus,
    pub created_by_id: String,
    pub updated_by_id: String,
}

#[derive(Debug, serde::Deserialize, derive_new::new)]
pub struct UpdateContent {
    pub id: String,
    pub category_id: Option<String>,
    pub fields: Option<serde_json::Value>,
    pub status: Option<ContentStatus>,
    pub updated_by_id: String,
}

#[async_trait::async_trait]
pub trait ContentRepository: Send + Sync {
    async fn get(&self) -> anyhow::Result<Vec<Content>>;
    async fn create(&self, data: CreateContent) -> anyhow::Result<Content>;
    async fn update(&self, data: UpdateContent) -> anyhow::Result<Content>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
