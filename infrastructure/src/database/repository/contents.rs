use anyhow::{Error, Result};
use async_trait::async_trait;
use derive_new::new;
use serde_json::Value;
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};

use domain::{
    model::{
        category::Category,
        content::{Content, ContentStatus, Field},
    },
    repository::content::{ContentRepository, CreateContent, UpdateContent},
};

use crate::database::ConnectionPool;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "content_status")]
pub enum ContentRowStatus {
    Draft,
    Published,
    Reserved,
    Unpublished,
}

impl From<ContentRowStatus> for ContentStatus {
    fn from(content_row_status: ContentRowStatus) -> Self {
        match content_row_status {
            ContentRowStatus::Draft => ContentStatus::Draft,
            ContentRowStatus::Published => ContentStatus::Published,
            ContentRowStatus::Reserved => ContentStatus::Reserved,
            ContentRowStatus::Unpublished => ContentStatus::Unpublished,
        }
    }
}

impl From<ContentStatus> for ContentRowStatus {
    fn from(content_status: ContentStatus) -> Self {
        match content_status {
            ContentStatus::Draft => ContentRowStatus::Draft,
            ContentStatus::Published => ContentRowStatus::Published,
            ContentStatus::Reserved => ContentRowStatus::Reserved,
            ContentStatus::Unpublished => ContentRowStatus::Unpublished,
        }
    }
}

#[derive(Debug, FromRow)]
pub struct ContentRow {
    pub id: Uuid,
    pub fields: Value,
    pub status: ContentRowStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub category_id: Uuid,
    pub category_name: String,
    pub category_api_identifier: String,
    pub category_description: Option<String>,
}

impl TryFrom<ContentRow> for Content {
    type Error = Error;
    fn try_from(row: ContentRow) -> Result<Self> {
        let ContentRow {
            id,
            fields,
            status,
            published_at,
            created_at,
            updated_at,
            category_id,
            category_name,
            category_api_identifier,
            category_description,
        } = row;

        let category = Category::try_new(
            category_id.into(),
            category_name,
            category_api_identifier,
            category_description,
        )?;
        let deserialized_fields: Vec<Field> = serde_json::from_value(fields)?;
        let published_at_str: Option<String> = match published_at {
            Some(datetime) => Some(datetime.to_string()),
            None => None,
        };

        Ok(Self {
            id: id.to_string(),
            category,
            status: status.into(),
            fields: deserialized_fields,
            published_at: published_at_str,
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
        })
    }
}

#[derive(new)]
pub struct ContentRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ContentRepository for ContentRepositoryImpl {
    async fn get(&self) -> Result<Vec<Content>> {
        let rows = sqlx::query_as::<_, ContentRow>(
            r#"
                SELECT
                    contents.*,
                    category.name AS category_name,
                    category.api_identifier AS category_api_identifier,
                    category.description AS category_description
                FROM contents 
                INNER JOIN category
                ON contents.category_id = category.id
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        rows.into_iter().map(Content::try_from).collect()
    }

    async fn create(&self, data: CreateContent) -> Result<Content> {
        let category_id = Uuid::parse_str(&data.category_id)?;
        let status: ContentRowStatus = data.status.into();

        let content_row = sqlx::query_as::<_, ContentRow>(
            r#"
                WITH inserted AS (
                    INSERT INTO
                        contents (
                            category_id,
                            fields,
                            status
                        )
                    VALUES ($1, $2, $3)
                    RETURNING
                        contents.*
                )
                SELECT
                    inserted.*,
                    category.name AS category_name,
                    category.api_identifier AS category_api_identifier,
                    category.description AS category_description
                FROM
                    inserted
                JOIN
                    category
                ON
                    inserted.category_id = category.id
            "#,
        )
        .bind(category_id)
        .bind(data.fields)
        .bind(status)
        .fetch_one(self.db.inner_ref())
        .await?;

        Content::try_from(content_row)
    }

    async fn update(&self, data: UpdateContent) -> Result<Content> {
        let UpdateContent {
            id, fields, status, ..
        } = data;

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE contents SET ");
        let mut separated = query_builder.separated(",");

        if let Some(fields) = fields {
            separated.push("fields = ");
            separated.push_bind_unseparated(fields);
        }

        if let Some(status) = status {
            separated.push("status = ");
            let content_row_status: ContentRowStatus = status.into();
            separated.push_bind_unseparated(content_row_status);
        }

        query_builder.push(" FROM category WHERE contents.category_id = category.id AND");

        let parsed_content_id = Uuid::parse_str(&id)?;
        query_builder.push(" contents.id = ");
        query_builder.push_bind(parsed_content_id);

        query_builder.push(" RETURNING contents.*, category.name AS category_name, category.api_identifier AS category_api_identifier, category.description AS category_description");

        let content_row = query_builder
            .build_query_as::<ContentRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        Content::try_from(content_row)
    }

    async fn delete(&self, id: String) -> Result<()> {
        let parsed_content_id = Uuid::parse_str(&id)?;

        sqlx::query(r#"DELETE FROM contents WHERE id = $1"#)
            .bind(parsed_content_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
