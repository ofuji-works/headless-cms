use anyhow::Result;
use derive_new::new;

#[derive(new)]
pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

pub struct AppConfig {
   pub database: DatabaseConfig
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let database = DatabaseConfig::new(
            std::env::var("DATABASE_HOST")?,
            std::env::var("DATABASE_PORT")?.parse()?,
            std::env::var("DATABASE_USER")?,
            std::env::var("DATABASE_PASSWORD")?,
            std::env::var("DATABASE_NAME")?
        ); 

        Ok(Self { database })
    }
}


