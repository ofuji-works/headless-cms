use domain::{
    model::content_model::ContentModel,
    repository::content_model::ContentModelRepository
};
use std::sync::Future;

use crate::database::config::{
    ConnectionPool,
    connect_database_with,
}

pub struct ContentModelPostgresRepository {
    db: ConnectionPool
} 

impl ContentModelRepository for ContentModelPostgresRepository {
    fn get (&self) -> impl Future<Output = Result<Vec<ContentModel>>> {
       sqlx::query("SELECT * FROM "); 
    } 
}
