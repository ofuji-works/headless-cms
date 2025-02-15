use domain::repository::category::{CategoryRepository, CreateCategory, UpdateCategory};

use crate::database::{repository::category::CategoryRepositoryImpl, ConnectionPool};

fn build_repository(pool: sqlx::PgPool) -> CategoryRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool);

    CategoryRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("category")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test]
fn create_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let create_data = CreateCategory::new(
        "sample1".into(),
        "sample1".into(),
        Some("sample1 content model".into()),
    );
    let result = repository.create(create_data).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("category")))]
fn update_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let categories = repository.get().await.unwrap();
    let category = categories.get(0).unwrap();

    let data = UpdateCategory::new(
        category.id.to_string(),
        Some("update-test".into()),
        None,
        None,
    );
    let result = repository.update(data).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../../fixtures", scripts("category")))]
fn delete_success(pool: sqlx::PgPool) {
    let repository = build_repository(pool);
    let categories = repository.get().await.unwrap();
    let category = categories.get(0).unwrap();

    let result = repository.delete(category.id.to_string()).await;

    assert_eq!(result.is_ok(), true);
}
