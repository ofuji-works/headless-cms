use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use domain::{
    model::category::Category,
    repository::category::{CategoryRepository, CreateCategory, UpdateCategory},
};

pub type CreateCategoryInput = CreateCategory;
pub type UpdateCategoryInput = UpdateCategory;

#[derive(new)]
pub struct CategoryUsecase {
    repository: Arc<dyn CategoryRepository>,
}

impl CategoryUsecase {
    pub async fn get(&self) -> Result<Vec<Category>> {
        self.repository.clone().get().await
    }

    pub async fn create(&self, input: CreateCategoryInput) -> Result<Category> {
        self.repository.clone().create(input).await
    }

    pub async fn update(&self, input: UpdateCategoryInput) -> Result<Category> {
        self.repository.clone().update(input).await
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.repository.clone().delete(id).await
    }
}
