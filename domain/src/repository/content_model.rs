use anyhow::Result;
use std::future::Future;

use crate::model::content_model::ContentModel;

pub trait ContentModelRepository {
    fn get(&self) -> impl Future<Output = Result<Vec<ContentModel>>> + Send + Sync;
    //fn find(&self) -> impl Future<Output = Result<Option<ContentModel>>> + Send + Sync;
    //fn create(&self) -> impl Future<Output = Result<ContentModel>> + Send + Sync;
    //fn update(&self) -> impl Future<Output = Result<ContentModel>> + Send + Sync;
}
