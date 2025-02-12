use std::sync::Arc;

use infrastructure::database::{
    connect_database_with,
    repository::{
        content_model::ContentModelRepositoryImpl, contents::ContentRepositoryImpl,
        health::HealthCheckRepositoryImpl,
    },
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
    content_repository: Arc<ContentRepositoryImpl>,
    content_model_repository: Arc<ContentModelRepositoryImpl>,
}

impl AppRegistry {
    pub fn new(config: AppConfig) -> Self {
        let pool = connect_database_with(config.database);

        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let content_repository = Arc::new(ContentRepositoryImpl::new(pool.clone()));
        let content_model_repository = Arc::new(ContentModelRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            content_repository,
            content_model_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<HealthCheckRepositoryImpl> {
        self.health_check_repository.clone()
    }

    pub fn content_repository(&self) -> Arc<ContentRepositoryImpl> {
        self.content_repository.clone()
    }

    pub fn content_model_repository(&self) -> Arc<ContentModelRepositoryImpl> {
        self.content_model_repository.clone()
    }
}
