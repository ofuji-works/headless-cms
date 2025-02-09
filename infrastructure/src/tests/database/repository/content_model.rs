use domain::repository::content_model::{ContentModelRepository, CreateContentModel};

use crate::database::{repository::content_model::ContentModelRepositoryImpl, ConnectionPool};

fn build_repository(pool: sqlx::PgPool) -> ContentModelRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool);

    ContentModelRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("content_model")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test]
fn create_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let create_data = CreateContentModel::new(
        "sample1".into(),
        "sample1".into(),
        Some("sample1 content model".into()),
    );
    let result = repository.create(create_data).await;

    assert_eq!(result.is_ok(), true);
}
