use anyhow::Result;
use derive_new::new;
use std::sync::Arc;

use domain::{
    model::content::Content,
    repository::content::{ContentRepository, CreateContent, GetContentQuery, UpdateContent},
};

#[derive(new)]
pub struct ContentUsecase {
    repository: Arc<dyn ContentRepository>,
}

pub type GetContentInput = GetContentQuery;
pub type CreateContentInput = CreateContent;
pub type UpdateContentInput = UpdateContent;

impl ContentUsecase {
    pub async fn get(&self, input: GetContentInput) -> Result<Vec<Content>> {
        self.repository.clone().get(input).await
    }

    pub async fn create(&self, input: CreateContentInput) -> Result<Content> {
        self.repository.clone().create(input).await
    }

    pub async fn update(&self, input: UpdateContentInput) -> Result<Content> {
        self.repository.clone().update(input).await
    }

    pub async fn delete(&self, id: String) -> Result<()> {
        self.repository.clone().delete(id).await
    }
}
