use anyhow::Result;
use async_trait::async_trait;

use crate::model::content_model::ContentModel;

#[async_trait]
pub trait ContentModelRepository: Send + Sync {
    async fn get(&self) -> Result<Vec<ContentModel>>;
    //fn find(&self) -> impl Future<Output = Result<Option<ContentModel>>> + Send + Sync;
    //fn create(&self) -> impl Future<Output = Result<ContentModel>> + Send + Sync;
    //fn update(&self) -> impl Future<Output = Result<ContentModel>> + Send + Sync;
}
