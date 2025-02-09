use anyhow::Result;

use crate::storage::StorageClient;

#[derive(derive_new::new)]
pub struct MediaRepositoryImpl {
    client: StorageClient,
}

impl MediaRepositoryImpl {
    pub async fn create_bucket(&self, bucket_name: String) -> Result<()> {
        let result = self
            .client
            .inner_ref()
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await?;

        Ok(())
    }

    fn delete_bucket() {}

    fn create_object() {}

    fn get_object() {}

    fn download_object() {}

    fn delete_object() {}
}
