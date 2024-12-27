use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use domain::{
    model::content::Content,
    repository::content::{ContentRepository, CreateContent, UpdateContent},
};

#[derive(new)]
pub struct ContentUsecase {
    repository: Arc<dyn ContentRepository>,
}

pub type CreateContentInput = CreateContent;
pub type UpdateContentInput = UpdateContent;

impl ContentUsecase {
    pub async fn get(&self) -> Result<Vec<Content>> {
        self.repository.clone().get().await
    }

    pub async fn create(&self, input: CreateContentInput) -> Result<()> {
        self.repository.clone().create(input).await
    }

    pub async fn update(&self, input: UpdateContentInput) -> Result<()> {
        self.repository.clone().update(input).await
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.repository.clone().delete(id).await
    }
}
