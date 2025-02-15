use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use serde::Deserialize;
use utoipa::{IntoParams, ToSchema};

use crate::model::category::Category;

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct CreateCategory {
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize, new, IntoParams, ToSchema)]
pub struct UpdateCategory {
    pub id: String,
    pub name: Option<String>,
    pub api_identifier: Option<String>,
    pub description: Option<String>,
}

#[mockall::automock]
#[async_trait]
pub trait CategoryRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<Category>>;
    async fn create(&self, data: CreateCategory) -> Result<Category>;
    async fn update(&self, data: UpdateCategory) -> Result<Category>;
    async fn delete(&self, id: String) -> Result<()>;
}
