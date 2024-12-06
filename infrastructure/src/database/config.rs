use sqlx::{postgres::PgConnectOptions, PgPool};

pub struct DatabaseConfig {
    pub host: String,
    pub port: u16,
    pub username: String,
    pub password: String,
    pub database: String,
}

impl From<DatabaseConfig> for PgConnectOptions {
    fn from(cfg: DatabaseConfig) -> Self {
        Self::new()
            .host(&cfg.host)
            .port(cfg.port)
            .username(&cfg.username)
            .password(&cfg.password)
            .database(&cfg.database)
    }
}

#[derive(Clone)]
pub struct ConnectionPool(PgPool)

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: DatabaseConfig) -> ConnectionPool {
    let pool = PgPool::connect_lazy_with(cfg.into());
    ConnectionPool(pool)
}
