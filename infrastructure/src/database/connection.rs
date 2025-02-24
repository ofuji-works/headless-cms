use shared::config::DatabaseConfig;

fn make_pg_connect_options(cfg: DatabaseConfig) -> sqlx::postgres::PgConnectOptions {
    sqlx::postgres::PgConnectOptions::new()
        .host(&cfg.host)
        .port(cfg.port)
        .username(&cfg.username)
        .password(&cfg.password)
        .database(&cfg.database)
}

#[derive(Clone, derive_new::new)]
pub struct ConnectionPool(sqlx::PgPool);

impl ConnectionPool {
    pub fn inner_ref(&self) -> &sqlx::PgPool {
        &self.0
    }
}

pub fn connect_database_with(cfg: DatabaseConfig) -> ConnectionPool {
    let pool = sqlx::PgPool::connect_lazy_with(make_pg_connect_options(cfg));

    ConnectionPool(pool)
}
