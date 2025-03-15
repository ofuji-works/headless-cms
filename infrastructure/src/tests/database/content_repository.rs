use domain::model::category::Category;
use domain::model::content::{ContentStatus, Field, FieldType};
use domain::model::tag::Tag;
use domain::model::user::User;
use domain::repository::category::{CategoryRepository, GetCategoryQuery};
use domain::repository::content::{
    ContentRepository, CreateContent, GetContentQuery, UpdateContent,
};
use domain::repository::tag::{GetTagQuery, TagRepository};
use domain::repository::user::{GetUserQuery, UserRepository};

use crate::database::category_repository::CategoryRepositoryImpl;
use crate::database::connection::ConnectionPool;
use crate::database::contents_repository::ContentRepositoryImpl;
use crate::database::tag_repository::TagRepositoryImpl;
use crate::database::user_repository::UserRepositoryImpl;

async fn get_user(pool: &sqlx::PgPool) -> User {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = UserRepositoryImpl::new(connection_pool);
    let query = GetUserQuery::new(1, 0);
    let rows = repo.get(query).await.unwrap();

    rows.get(0).unwrap().clone()
}

async fn get_category(pool: &sqlx::PgPool) -> Category {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = CategoryRepositoryImpl::new(connection_pool);
    let query = GetCategoryQuery::default();
    let categories = repo.get(query).await.unwrap();

    categories.get(0).unwrap().clone()
}

async fn get_tags(pool: &sqlx::PgPool) -> Vec<Tag> {
    let connection_pool = ConnectionPool::new(pool.clone());
    let repo = TagRepositoryImpl::new(connection_pool);
    let query = GetTagQuery::default();

    repo.get(query).await.unwrap()
}

fn build_repository(pool: &sqlx::PgPool) -> ContentRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool.clone());

    ContentRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("users", "content", "tags", "content_tags")
))]
fn get_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);
    let query = GetContentQuery::default();
    let result = repository.get(query).await;

    println!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("users", "category", "tags")))]
fn create_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let category = get_category(&pool).await;
    let tag_ids = get_tags(&pool)
        .await
        .iter()
        .map(|t| t.id.to_string())
        .collect::<Vec<_>>();

    let content_repository = build_repository(&pool);
    let field = Field::new(FieldType::ShortText, "title".into());
    let fields = serde_json::to_value(vec![field]).unwrap();
    let create_content = CreateContent::new(
        "title".into(),
        category.id.to_string(),
        fields,
        tag_ids,
        ContentStatus::Draft,
        user.id.clone(),
        user.id.clone(),
    );
    let result = content_repository.create(create_content).await;

    println!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("users", "content", "tags", "content_tags")
))]
fn update_success(pool: sqlx::PgPool) {
    let user = get_user(&pool).await;
    let repository = build_repository(&pool);
    let query = GetContentQuery::default();
    let contents = repository.get(query).await.unwrap();
    let content = contents.get(0).unwrap();

    let update_content = UpdateContent::new(
        content.id.to_string(),
        Some("changed".into()),
        None,
        None,
        None,
        Some(ContentStatus::Reserved),
        user.id,
    );

    let result = repository.update(update_content).await;

    println!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}

#[sqlx::test(fixtures(
    path = "../fixtures",
    scripts("users", "content", "tags", "content_tags")
))]
fn delete_success(pool: sqlx::PgPool) {
    let repository = build_repository(&pool);
    let query = GetContentQuery::default();
    let contents = repository.get(query).await.unwrap();
    let content = contents.get(0).unwrap();

    let result = repository.delete(content.id.clone()).await;

    println!("{:?}", result);

    assert_eq!(result.is_ok(), true);
}
