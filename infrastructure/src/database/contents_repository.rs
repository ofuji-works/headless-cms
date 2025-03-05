use std::str::FromStr;

use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;

use domain::model::content::{
    Categories, Content, ContentStatus, CreatedBy, Field, Tags, UpdatedBy,
};
use domain::repository::content::{ContentRepository, CreateContent, UpdateContent};

use crate::database::connection::ConnectionPool;

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

#[derive(Debug, sqlx::FromRow)]
pub struct ContentRow {
    pub id: Uuid,
    pub fields: serde_json::Value,
    pub tags: serde_json::Value,
    pub status: ContentRowStatus,
    pub published_at: Option<DateTime<Utc>>,
    pub created_by_id: Uuid,
    pub created_by_name: String,
    pub updated_by_id: Uuid,
    pub updated_by_name: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub category_id: Uuid,
    pub category_name: String,
}

impl TryFrom<ContentRow> for Content {
    type Error = anyhow::Error;
    fn try_from(row: ContentRow) -> anyhow::Result<Self> {
        let ContentRow {
            id,
            fields,
            tags,
            status,
            published_at,
            created_at,
            updated_at,
            category_id,
            category_name,
            created_by_id,
            created_by_name,
            updated_by_id,
            updated_by_name,
            ..
        } = row;

        let categories = Categories::new(category_id.into(), category_name);
        let deserialized_fields: Vec<Field> = serde_json::from_value(fields)?;
        let deserialized_tags: Vec<Tags> = serde_json::from_value(tags)?;
        let published_at_str: Option<String> = match published_at {
            Some(datetime) => Some(datetime.to_string()),
            None => None,
        };
        let created_by = CreatedBy::new(created_by_id.into(), created_by_name);
        let updated_by = UpdatedBy::new(updated_by_id.into(), updated_by_name);

        Ok(Self {
            id: id.to_string(),
            categories,
            status: status.into(),
            fields: deserialized_fields,
            tags: deserialized_tags,
            published_at: published_at_str,
            created_at: created_at.to_string(),
            updated_at: updated_at.to_string(),
            created_by,
            updated_by,
        })
    }
}

#[derive(derive_new::new)]
pub struct ContentRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl ContentRepository for ContentRepositoryImpl {
    async fn get(&self) -> anyhow::Result<Vec<Content>> {
        let rows = sqlx::query_as::<_, ContentRow>(
            r#"
                SELECT
                    contents.*,
                    category.name AS category_name,
                    category.api_identifier AS category_api_identifier,
                    category.description AS category_description,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name,
                    COALESCE(json_agg(json_build_object('id', tags.id, 'name', tags.name)) 
                        FILTER (WHERE tags.id IS NOT NULL), '[]'::json) AS tags
                FROM
                    contents 
                JOIN
                    category ON contents.category_id = category.id
                JOIN
                    users AS created_by ON created_by.id = contents.created_by
                JOIN
                    users AS updated_by ON updated_by.id = contents.updated_by
                JOIN
                    content_tags ON contents.id = content_tags.content_id
                JOIN
                    tags ON tags.id = content_tags.tag_id
                GROUP BY
                    contents.id,
                    contents.fields,
                    contents.status,
                    contents.category_id,
                    contents.published_at,
                    contents.created_at,
                    contents.updated_at,
                    contents.created_by,
                    contents.updated_by,
                    category.name,
                    category.api_identifier,
                    category.description,
                    created_by.id,
                    created_by.name,
                    updated_by.id,
                    updated_by.name
                ORDER BY
                    contents.created_at DESC
            "#,
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        rows.into_iter().map(Content::try_from).collect()
    }

    async fn create(&self, data: CreateContent) -> anyhow::Result<Content> {
        let CreateContent {
            category_id,
            fields,
            status,
            tag_ids,
            created_by_id,
            updated_by_id,
        } = data;

        let category_id = Uuid::parse_str(&category_id)?;
        let status: ContentRowStatus = status.into();
        let created_by = Uuid::parse_str(&created_by_id)?;
        let updated_by = Uuid::parse_str(&updated_by_id)?;
        let tag_uuids: Vec<uuid::Uuid> = tag_ids
            .into_iter()
            .map(|id| uuid::Uuid::from_str(&id))
            .collect::<Result<Vec<_>, _>>()?;

        let content_row = sqlx::query_as::<_, ContentRow>(
            r#"
                WITH inserted AS (
                    INSERT INTO
                        contents (
                            category_id,
                            fields,
                            status,
                            created_by,
                            updated_by
                        )
                    VALUES ($1, $2, $3, $4, $5)
                    RETURNING
                        contents.*
                ),
                inserted_tags AS (
                    INSERT INTO
                        content_tags (content_id, tag_id)
                    SELECT
                        inserted.id,
                        tags.id
                    FROM
                        inserted
                    JOIN
                        tags ON tags.id = ANY($6)
                    RETURNING
                        *
                )
                SELECT
                    inserted.*,
                    category.name AS category_name,
                    category.api_identifier AS category_api_identifier,
                    category.description AS category_description,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name,
                    COALESCE(json_agg(json_build_object('id', tags.id, 'name', tags.name)) 
                        FILTER (WHERE tags.id IS NOT NULL), '[]'::json) AS tags
                FROM
                    inserted
                JOIN
                    category ON category.id = inserted.category_id 
                JOIN
                    users AS created_by ON created_by.id = inserted.created_by
                JOIN
                    users AS updated_by ON updated_by.id = inserted.updated_by
                JOIN
                    inserted_tags ON inserted.id = inserted_tags.content_id
                JOIN
                    tags ON tags.id = inserted_tags.tag_id
                GROUP BY
                    inserted.id,
                    inserted.fields,
                    inserted.status,
                    inserted.category_id,
                    inserted.published_at,
                    inserted.created_at,
                    inserted.updated_at,
                    inserted.created_by,
                    inserted.updated_by,
                    category.name,
                    category.api_identifier,
                    category.description,
                    created_by.id,
                    created_by.name,
                    updated_by.id,
                    updated_by.name
            "#,
        )
        .bind(category_id)
        .bind(fields)
        .bind(status)
        .bind(created_by)
        .bind(updated_by)
        .bind(tag_uuids)
        .fetch_one(self.db.inner_ref())
        .await?;

        Content::try_from(content_row)
    }

    async fn update(&self, data: UpdateContent) -> anyhow::Result<Content> {
        let UpdateContent {
            id,
            fields,
            status,
            updated_by_id,
            ..
        } = data;

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
            "
                WITH updated AS (
                    UPDATE contents SET
            ",
        );
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

        let parsed_updated_by = Uuid::parse_str(&updated_by_id)?;
        separated.push("updated_by = ");
        separated.push_bind_unseparated(parsed_updated_by);

        let parsed_content_id = Uuid::parse_str(&id)?;
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(parsed_content_id);

        query_builder.push(
            "
                    RETURNING *
                )
                SELECT
                    updated.*,
                    category.name AS category_name,
                    category.api_identifier AS category_api_identifier,
                    category.description AS category_description,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name,
                    COALESCE(json_agg(json_build_object('id', tags.id, 'name', tags.name)) 
                        FILTER (WHERE tags.id IS NOT NULL), '[]'::json) AS tags
                FROM
                    updated
                JOIN
                    category ON category.id = updated.category_id
                JOIN
                    users AS created_by ON created_by.id = updated.created_by
                JOIN
                    users AS updated_by ON updated_by.id = updated.updated_by
                JOIN
                    content_tags ON updated.id = content_tags.content_id
                JOIN
                    tags ON tags.id = content_tags.tag_id
                GROUP BY
                    updated.id,
                    updated.fields,
                    updated.status,
                    updated.category_id,
                    updated.published_at,
                    updated.created_at,
                    updated.updated_at,
                    updated.created_by,
                    updated.updated_by,
                    category.name,
                    category.api_identifier,
                    category.description,
                    created_by.id,
                    created_by.name,
                    updated_by.id,
                    updated_by.name
            ",
        );

        let content_row = query_builder
            .build_query_as::<ContentRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        Content::try_from(content_row)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let parsed_content_id = Uuid::parse_str(&id)?;

        sqlx::query(r#"DELETE FROM contents WHERE id = $1"#)
            .bind(parsed_content_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
