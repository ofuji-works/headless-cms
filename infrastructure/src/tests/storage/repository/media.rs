use shared::config::StorageConfig;

use crate::storage::{repository::media::MediaRepositoryImpl, StorageClient};

#[tokio::test]
async fn create_bucket_test() -> anyhow::Result<()> {
    let config = StorageConfig::new(
        std::env::var("STORAGE_ACCESS_KEY")?,
        std::env::var("STORAGE_SECRET_KEY")?,
        std::env::var("STORAGE_REGION")?,
        std::env::var("STORAGE_ENDPOINT")?,
    );
    let cli = StorageClient::new(config);
    let repo = MediaRepositoryImpl::new(cli);

    let result = repo.create_bucket("test".into()).await;
    assert_eq!(result.is_ok(), true);

    repo.delete_bucket(result?).await?;

    Ok(())
}
