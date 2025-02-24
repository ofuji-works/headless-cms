use shared::config::StorageConfig;

pub struct StorageClient(aws_sdk_s3::Client);

impl StorageClient {
    pub fn inner_ref(&self) -> &aws_sdk_s3::Client {
        &self.0
    }

    pub fn new(config: StorageConfig) -> Self {
        tracing_subscriber::fmt::init();
        let credentials = aws_sdk_s3::config::Credentials::new(
            config.access_key,
            config.secret_key,
            None,
            None,
            "",
        );

        let region = aws_sdk_s3::config::Region::new(config.region);

        let aws_config = aws_sdk_s3::Config::builder()
            .credentials_provider(credentials)
            .endpoint_url(config.endpoint)
            .region(region)
            .behavior_version_latest()
            .force_path_style(true)
            .build();

        let client = aws_sdk_s3::Client::from_conf(aws_config);

        Self(client)
    }
}
