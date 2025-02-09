use domain::model::content::{ContentStatus, Field, FieldType};
use domain::repository::content::{ContentRepository, CreateContent, UpdateContent};
use domain::repository::content_model::ContentModelRepository;

use crate::database::repository::content_model::ContentModelRepositoryImpl;
use crate::database::repository::contents::ContentRepositoryImpl;
use crate::database::ConnectionPool;

fn build_content_repository(pool: &sqlx::PgPool) -> ContentRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    ContentRepositoryImpl::new(connection_pool)
}

fn build_content_model_repository(pool: &sqlx::PgPool) -> ContentModelRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    ContentModelRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("content")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_content_repository(&pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("content_model")))]
fn create_success(pool: sqlx::PgPool) {
    let content_model_repository = build_content_model_repository(&pool);
    let content_models = content_model_repository.get().await.unwrap();
    let content_model = content_models.get(0).unwrap();

    let content_repository = build_content_repository(&pool);
    let field = Field::new(FieldType::ShortText, "title".into());
    let fields = serde_json::to_value(vec![field]).unwrap();
    let create_content =
        CreateContent::new(content_model.id.to_string(), fields, ContentStatus::Draft);
    let result = content_repository.create(create_content).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("content")))]
fn update_success(pool: sqlx::PgPool) {
    let repository = build_content_repository(&pool);
    let contents = repository.get().await.unwrap();
    let content = contents.get(0).unwrap();

    let update_content = UpdateContent::new(
        content.id.to_string(),
        None,
        None,
        Some(ContentStatus::Reserved),
    );

    let result = repository.update(update_content).await;

    assert_eq!(result.is_ok(), true);
}
