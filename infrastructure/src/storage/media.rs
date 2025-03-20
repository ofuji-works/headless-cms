use aws_sdk_s3::operation::create_bucket::CreateBucketOutput;

use crate::storage::client::StorageClient;

#[derive(Debug)]
pub struct Bucket {
    name: String,
}

impl TryFrom<CreateBucketOutput> for Bucket {
    type Error = anyhow::Error;
    fn try_from(output: CreateBucketOutput) -> anyhow::Result<Self> {
        let name: String = match output.location() {
            Some(location) => Ok(location.replace("/", "")),
            None => Err(anyhow::anyhow!("None CreateBucketOutput.name")),
        }?;

        Ok(Self { name })
    }
}

#[derive(derive_new::new, Debug)]
pub struct MediaRepositoryImpl {
    client: StorageClient,
}

impl MediaRepositoryImpl {
    #[tracing::instrument]
    pub async fn create_bucket(&self, bucket_name: String) -> anyhow::Result<Bucket> {
        let result = self
            .client
            .inner_ref()
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await?;

        tracing::info!("{:?}", result);

        Bucket::try_from(result)
    }

    #[tracing::instrument]
    pub async fn delete_bucket(&self, bucket: Bucket) -> anyhow::Result<()> {
        let result = self
            .client
            .inner_ref()
            .delete_bucket()
            .bucket(bucket.name)
            .send()
            .await;

        if result.is_err() {
            tracing::error!("failed delete bucket");
            anyhow::bail!("failed delete bucket")
        }

        tracing::info!("{:?}", result);

        Ok(())
    }

    fn create_object() {}

    fn get_object() {}

    fn download_object() {}

    fn delete_object() {}
}
