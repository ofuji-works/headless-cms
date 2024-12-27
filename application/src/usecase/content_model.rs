use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use domain::{
    model::content_model::ContentModel,
    repository::content_model::{ContentModelRepository, CreateContentModel, UpdateContentModel},
};

pub type CreateContentModelInput = CreateContentModel;
pub type UpdateContentModelInput = UpdateContentModel;

#[derive(new)]
pub struct ContentModelUsecase {
    repository: Arc<dyn ContentModelRepository>,
}

impl ContentModelUsecase {
    pub async fn get(&self) -> Result<Vec<ContentModel>> {
        self.repository.clone().get().await
    }

    pub async fn create(&self, input: CreateContentModelInput) -> Result<()> {
        self.repository.clone().create(input).await
    }

    pub async fn update(&self, input: UpdateContentModelInput) -> Result<()> {
        self.repository.clone().update(input).await
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.repository.clone().delete(id).await
    }
}
