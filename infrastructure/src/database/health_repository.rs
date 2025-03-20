use crate::database::connection::ConnectionPool;

#[derive(derive_new::new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

impl HealthCheckRepositoryImpl {
    pub async fn check_db(&self) -> bool {
        let result = sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok();

        tracing::info!("{:?}", result);

        result
    }
}
