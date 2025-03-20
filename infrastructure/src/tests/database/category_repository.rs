use domain::repository::category::{
    CategoryRepository, CreateCategory, GetCategoryQuery, UpdateCategory,
};
use shared::logger::logger_init_info;

use crate::database::category_repository::CategoryRepositoryImpl;
use crate::database::connection::ConnectionPool;

fn build_repository(pool: &sqlx::PgPool) -> CategoryRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    CategoryRepositoryImpl::new(connection_pool)
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("category")))]
fn get_success(pool: sqlx::PgPool) {
    logger_init_info();
    let repository = build_repository(&pool);
    let query = GetCategoryQuery::default();
    let result = repository.get(query).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test]
fn create_success(pool: sqlx::PgPool) {
    logger_init_info();
    let repo = build_repository(&pool);
    let create_data = CreateCategory::new(
        "sample1".into(),
        "sample1".into(),
        Some("sample1 content model".into()),
    );
    let result = repo.create(create_data).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("category")))]
fn update_success(pool: sqlx::PgPool) {
    logger_init_info();
    let repo = build_repository(&pool);
    let query = GetCategoryQuery::default();
    let categories = repo.get(query).await.unwrap();
    let category = categories.get(0).unwrap();

    let data = UpdateCategory::new(
        category.id.to_string(),
        Some("update-test".into()),
        None,
        None,
    );
    let result = repo.update(data).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("category")))]
fn delete_success(pool: sqlx::PgPool) {
    logger_init_info();
    let repository = build_repository(&pool);
    let query = GetCategoryQuery::default();
    let categories = repository.get(query).await.unwrap();
    let category = categories.get(0).unwrap();

    let result = repository.delete(category.id.to_string()).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}
