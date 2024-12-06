use std::sync::Arc;

use infrastructure::database::{repository::health::HealthCheckRepositoryImpl, ConnectionPool};

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>
}

impl AppRegistry {
    pub fn new (pool: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool));

        Self {
            health_check_repository
        }
    }

    pub fn health_check_repository(&self) -> Arc<HealthCheckRepositoryImpl> {
        self.health_check_repository.clone()
    }
}
