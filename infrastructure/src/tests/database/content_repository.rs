use domain::model::category::Category;
use domain::model::content::{ContentStatus, Field, FieldType};
use domain::model::user::User;
use domain::repository::category::CategoryRepository;
use domain::repository::content::{ContentRepository, CreateContent, UpdateContent};
use domain::repository::user::{GetUserQuery, UserRepository};

use crate::database::category_repository::CategoryRepositoryImpl;
use crate::database::connection::ConnectionPool;
use crate::database::contents_repository::ContentRepositoryImpl;
use crate::database::user_repository::UserRepositoryImpl;

async fn get_user(pool: &sqlx::PgPool) -> User {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(connection_pool);
    let query = GetUserQuery::new(0, 1);
    let rows = repo.get(query).await.unwrap();

    rows.get(0).unwrap().clone()
}

async fn get_category(pool: &sqlx::PgPool) -> Category {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = CategoryRepositoryImpl::new(connection_pool);
    let categories = repo.get().await.unwrap();

    categories.get(0).unwrap().clone()
}

fn build_repository(pool: &sqlx::PgPool) -> ContentRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    ContentRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "content")))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "category")))]
fn create_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let category = get_category(&pool).await;

    let content_repository = build_repository(&pool);
    let field = Field::new(FieldType::ShortText, "title".into());
    let fields = serde_json::to_value(vec![field]).unwrap();
    let create_content = CreateContent::new(
        category.id.to_string(),
        fields,
        ContentStatus::Draft,
        user.id.clone(),
        user.id.clone(),
    );
    let result = content_repository.create(create_content).await;

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "content")))]
fn update_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let repository = build_repository(&pool);
    let contents = repository.get().await.unwrap();
    let content = contents.get(0).unwrap();

    let update_content = UpdateContent::new(
        content.id.to_string(),
        None,
        None,
        Some(ContentStatus::Reserved),
        user.id,
    );

    let result = repository.update(update_content).await;

    assert_eq!(result.is_ok(), true);
}
