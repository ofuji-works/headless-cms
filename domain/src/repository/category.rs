use crate::model::category::Category;

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CreateCategory {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
    pub created_by_id: String,
    pub updated_by_id: String,
}

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UpdateCategory {
    pub id: String,
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
    pub updated_by_id: String,
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn get(&self) -> anyhow::Result<Vec<Category>>;
    async fn create(&self, data: CreateCategory) -> anyhow::Result<Category>;
    async fn update(&self, data: UpdateCategory) -> anyhow::Result<Category>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
