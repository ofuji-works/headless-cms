pub mod repository;

use derive_new::new;
use shared::config::DatabaseConfig;
use sqlx::{postgres::PgConnectOptions, PgPool};

fn make_pg_connect_options(cfg: DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone, new)]
pub struct ConnectionPool(PgPool);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: DatabaseConfig) -> ConnectionPool {
    let pool = PgPool::connect_lazy_with(make_pg_connect_options(cfg));

    ConnectionPool(pool)
}
