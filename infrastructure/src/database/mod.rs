pub mod repository;

use sqlx::{postgres::PgConnectOptions, PgPool};
use shared::config::DatabaseConfig;

fn make_pg_connect_options(cfg: DatabaseConfig) -> PgConnectOptions {
    PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}


#[derive(Clone)]
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

