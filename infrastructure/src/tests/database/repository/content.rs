use domain::model::content::{ContentStatus, Field, FieldType};
use domain::repository::category::CategoryRepository;
use domain::repository::content::{ContentRepository, CreateContent, UpdateContent};

use crate::database::repository::category::CategoryRepositoryImpl;
use crate::database::repository::contents::ContentRepositoryImpl;
use crate::database::ConnectionPool;

fn build_content_repository(pool: &sqlx::PgPool) -> ContentRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    ContentRepositoryImpl::new(connection_pool)
}

fn build_category_repository(pool: &sqlx::PgPool) -> CategoryRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    CategoryRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("content")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_content_repository(&pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("category")))]
fn create_success(pool: sqlx::PgPool) {
    let category_repository = build_category_repository(&pool);
    let categorys = category_repository.get().await.unwrap();
    let category = categorys.get(0).unwrap();

    let content_repository = build_content_repository(&pool);
    let field = Field::new(FieldType::ShortText, "title".into());
    let fields = serde_json::to_value(vec![field]).unwrap();
    let create_content = CreateContent::new(category.id.to_string(), fields, ContentStatus::Draft);
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

    println!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}
