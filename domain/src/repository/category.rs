use crate::model::category::Category;

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct GetCategoryQuery {
    pub limit: i32,
    pub offset: i32,
}

impl Default for GetCategoryQuery {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct CreateCategory {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
}

#[derive(Debug, serde::Deserialize, derive_new::new, utoipa::IntoParams, utoipa::ToSchema)]
pub struct UpdateCategory {
    pub id: String,
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
}

#[mockall::automock]
#[async_trait::async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn get(&self, query: GetCategoryQuery) -> anyhow::Result<Vec<Category>>;
    async fn create(&self, data: CreateCategory) -> anyhow::Result<Category>;
    async fn update(&self, data: UpdateCategory) -> anyhow::Result<Category>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
