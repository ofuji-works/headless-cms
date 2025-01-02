use anyhow::Result;

use domain::{
    model::field_meta::{FieldMeta, FieldType},
    repository::content_model::{ContentModelRepository, CreateContentModel},
};

use crate::database::{repository::content_model::ContentModelRepositoryImpl, ConnectionPool};

fn build_repository(pool: sqlx::PgPool) -> ContentModelRepositoryImpl {
    let connection_pool = ConnectionPool::new(pool);

    ContentModelRepositoryImpl::new(connection_pool)
}

#[sqlx::test(fixtures(path = "../fixtures", scripts("content_model")))]
fn get_success(pool: sqlx::PgPool) -> Result<()> {
    let repository = build_repository(pool);
    let result = repository.get().await;

    assert_eq!(result.is_ok(), true);

    Ok(())
}

#[sqlx::test]
fn create_success(pool: sqlx::PgPool) -> Result<()> {
    let repository = build_repository(pool);
    let field = FieldMeta::try_new("title".into(), "title".into(), FieldType::ShortText, true)?;
    let serialized_field = serde_json::to_value(field)?;
    let create_data = CreateContentModel::new(
        "sample1".into(),
        "sample1".into(),
        Some("sample1 content model".into()),
        serialized_field,
    );
    let result = repository.create(create_data).await;

    assert_eq!(result.is_ok(), true);

    Ok(())
}
