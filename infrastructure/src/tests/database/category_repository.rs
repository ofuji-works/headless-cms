use domain::model::user::User;
use domain::repository::category::{CategoryRepository, CreateCategory, UpdateCategory};
use domain::repository::user::{GetUserQuery, UserRepository};

use crate::database::category_repository::CategoryRepositoryImpl;
use crate::database::connection::ConnectionPool;
use crate::database::user_repository::UserRepositoryImpl;

async fn get_user(pool: &sqlx::PgPool) -> User {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(connection_pool);
    let query = GetUserQuery::new(0, 1);
    let rows = repo.get(query).await.unwrap();

    rows.get(0).unwrap().clone()
}

fn build_repository(pool: &sqlx::PgPool) -> CategoryRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    CategoryRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "category")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users")))]
fn create_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let repo = build_repository(&pool);
    let create_data = CreateCategory::new(
        "sample1".into(),
        "sample1".into(),
        Some("sample1 content model".into()),
        user.id.clone(),
        user.id.clone(),
    );
    let result = repo.create(create_data).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "category")))]
fn update_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let repo = build_repository(&pool);
    let categories = repo.get().await.unwrap();
    let category = categories.get(0).unwrap();

    let data = UpdateCategory::new(
        category.id.to_string(),
        Some("update-test".into()),
        None,
        None,
        user.id,
    );
    let result = repo.update(data).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "category")))]
fn delete_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);
    let categories = repository.get().await.unwrap();
    let category = categories.get(0).unwrap();

    let result = repository.delete(category.id.to_string()).await;

    assert_eq!(result.is_ok(), true);
}
