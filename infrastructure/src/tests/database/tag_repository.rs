use domain::repository::tag::{CreateTag, GetTagQuery, TagRepository, UpdateTag};

use crate::database::connection::ConnectionPool;
use crate::database::tag_repository::TagRepositoryImpl;

fn build_repository(pool: &sqlx::PgPool) -> TagRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    TagRepositoryImpl::new(connection_pool)
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("tags")))]
async fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);

    let query = GetTagQuery::new(10, 0);

    let result = repository.get(query).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test]
async fn create_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);

    let tag = CreateTag::new("Rust".into(), Some("Rust tags".into()));

    let result = repository.create(tag).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("tags")))]
async fn update_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);

    let query = GetTagQuery::new(10, 0);
    let rows = repository.get(query).await.unwrap();
    let row = rows.get(0).unwrap();
    tracing::info!("{:?}", row);

    let tag = UpdateTag::new(
        row.id.clone(),
        Some("NoRust".into()),
        Some("No Rust Tags".into()),
    );

    let result = repository.update(tag).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[tracing::instrument]
#[sqlx::test(fixtures(path = "../fixtures", scripts("tags")))]
async fn delete_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);

    let query = GetTagQuery::new(10, 0);
    let rows = repository.get(query).await.unwrap();
    let row = rows.get(0).unwrap();

    tracing::info!("{:?}", row);

    let result = repository.delete(row.id.clone()).await;

    tracing::info!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}
