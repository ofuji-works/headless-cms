use std::future::Future;

use crate::model::content_model::ContentModel;

pub trait ContentModelRepository {
    fn get () -> impl Future<Output = Vec<ContentModel>> + Send + Sync;
    fn find () -> impl Future<Output = ContentModel> + Send + Sync;
    fn create() -> impl Future<Output = ContentModel> + Send + Sync;
}
