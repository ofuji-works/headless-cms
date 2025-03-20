use anyhow::Result;
use derive_new::new;

#[derive(new, Debug)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

#[derive(new, Debug)]
pub struct StorageConfig {
    pub access_key: String,
    pub secret_key: String,
    pub region: String,
    pub endpoint: String,
}

pub struct AppConfig {
    pub database: DatabaseConfig,
    pub storage: StorageConfig,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig::new(
            std::env::var("DATABASE_HOST")?,
            std::env::var("POSTGRES_PORT")?.parse()?,
            std::env::var("DATABASE_USERNAME")?,
            std::env::var("DATABASE_PASSWORD")?,
            std::env::var("DATABASE_NAME")?,
        );

        let storage = StorageConfig::new(
            std::env::var("STORAGE_ACCESS_KEY")?,
            std::env::var("STORAGE_SECRET_KEY")?,
            std::env::var("STORAGE_REGION")?,
            std::env::var("STORAGE_ENDPOINT")?,
        );

        Ok(Self { database, storage })
    }
}
