use std::sync::Arc;

use infrastructure::database::{
    repository::{content_model::ContentModelRepositoryImpl, health::HealthCheckRepositoryImpl},
    ConnectionPool,
};

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
    content_model_repository: Arc<ContentModelRepositoryImpl>,
}

impl AppRegistry {
    pub fn new(pool: ConnectionPool) -> Self {
        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let content_model_repository = Arc::new(ContentModelRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            content_model_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<HealthCheckRepositoryImpl> {
        self.health_check_repository.clone()
    }

    pub fn content_model_repository(&self) -> Arc<ContentModelRepositoryImpl> {
        self.content_model_repository.clone()
    }
}
