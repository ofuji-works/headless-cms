use aws_sdk_s3::operation::create_bucket::CreateBucketOutput;

use crate::storage::StorageClient;

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

#[derive(derive_new::new)]
pub struct MediaRepositoryImpl {
    client: StorageClient,
}

impl MediaRepositoryImpl {
    pub async fn create_bucket(&self, bucket_name: String) -> anyhow::Result<Bucket> {
        let result = self
            .client
            .inner_ref()
            .create_bucket()
            .bucket(bucket_name)
            .send()
            .await?;

        Bucket::try_from(result)
    }

    pub async fn delete_bucket(&self, bucket: Bucket) -> anyhow::Result<()> {
        let result = self
            .client
            .inner_ref()
            .delete_bucket()
            .bucket(bucket.name)
            .send()
            .await;

        if result.is_err() {
            anyhow::bail!("failed delete bucket")
        }

        Ok(())
    }

    fn create_object() {}

    fn get_object() {}

    fn download_object() {}

    fn delete_object() {}
}
