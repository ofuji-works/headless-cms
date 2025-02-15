use std::sync::Arc;

use infrastructure::database::{
    connect_database_with,
    repository::{
        category::CategoryRepositoryImpl, contents::ContentRepositoryImpl,
        health::HealthCheckRepositoryImpl,
    },
};
use shared::config::AppConfig;

#[derive(Clone)]
pub struct AppRegistry {
    health_check_repository: Arc<HealthCheckRepositoryImpl>,
    content_repository: Arc<ContentRepositoryImpl>,
    category_repository: Arc<CategoryRepositoryImpl>,
}

impl AppRegistry {
    pub fn new(config: AppConfig) -> Self {
        let pool = connect_database_with(config.database);

        let health_check_repository = Arc::new(HealthCheckRepositoryImpl::new(pool.clone()));
        let content_repository = Arc::new(ContentRepositoryImpl::new(pool.clone()));
        let category_repository = Arc::new(CategoryRepositoryImpl::new(pool.clone()));

        Self {
            health_check_repository,
            content_repository,
            category_repository,
        }
    }

    pub fn health_check_repository(&self) -> Arc<HealthCheckRepositoryImpl> {
        self.health_check_repository.clone()
    }

    pub fn content_repository(&self) -> Arc<ContentRepositoryImpl> {
        self.content_repository.clone()
    }

    pub fn category_repository(&self) -> Arc<CategoryRepositoryImpl> {
        self.category_repository.clone()
    }
}
