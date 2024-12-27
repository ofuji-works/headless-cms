use derive_new::new;

use crate::database::ConnectionPool;

#[derive(new)]
pub struct HealthCheckRepositoryImpl {
    db: ConnectionPool,
}

impl HealthCheckRepositoryImpl {
    pub async fn check_db(&self) -> bool {
        sqlx::query("SELECT 1")
            .fetch_one(self.db.inner_ref())
            .await
            .is_ok()
    }
}
