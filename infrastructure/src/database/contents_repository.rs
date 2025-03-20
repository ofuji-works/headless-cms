use std::str::FromStr;

use domain::model::content::{
    Categories, Content, ContentStatus, CreatedBy, Field, Tags, UpdatedBy,
};
use domain::repository::content::{
    ContentRepository, CreateContent, GetContentQuery, UpdateContent,
};

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
pub struct ContentsRow {
    pub id: uuid::Uuid,
    pub title: String,
    pub fields: serde_json::Value,
    pub tags: serde_json::Value,
    pub status: ContentRowStatus,
    pub published_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
    pub created_by_id: uuid::Uuid,
    pub created_by_name: String,
    pub updated_by_id: uuid::Uuid,
    pub updated_by_name: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub category_id: uuid::Uuid,
    pub category_name: String,
}

impl TryFrom<ContentsRow> for Content {
    type Error = anyhow::Error;
    fn try_from(row: ContentsRow) -> anyhow::Result<Self> {
        let ContentsRow {
            id,
            title,
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
            title,
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

#[derive(Debug, sqlx::FromRow)]
pub struct TagRow {
    pub id: uuid::Uuid,
    pub name: String,
}

impl From<TagRow> for Tags {
    fn from(value: TagRow) -> Self {
        let TagRow { id, name } = value;

        Self {
            id: id.into(),
            name,
        }
    }
}

#[derive(Debug, sqlx::FromRow)]
pub struct ContentRow {
    pub id: uuid::Uuid,
    pub title: String,
    pub fields: serde_json::Value,
    pub status: ContentRowStatus,
    pub published_at: Option<sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>>,
    pub created_by_id: uuid::Uuid,
    pub created_by_name: String,
    pub updated_by_id: uuid::Uuid,
    pub updated_by_name: String,
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    pub category_id: uuid::Uuid,
    pub category_name: String,
}

fn try_new_content(content_row: ContentRow, tag_rows: Vec<TagRow>) -> anyhow::Result<Content> {
    let ContentRow {
        id,
        title,
        fields,
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
    } = content_row;

    let categories = Categories::new(category_id.into(), category_name);
    let deserialized_fields: Vec<Field> = serde_json::from_value(fields)?;
    let published_at_str: Option<String> = match published_at {
        Some(datetime) => Some(datetime.to_string()),
        None => None,
    };
    let created_by = CreatedBy::new(created_by_id.into(), created_by_name);
    let updated_by = UpdatedBy::new(updated_by_id.into(), updated_by_name);

    Ok(Content {
        id: id.to_string(),
        title,
        categories,
        status: status.into(),
        fields: deserialized_fields,
        tags: tag_rows.into_iter().map(Tags::from).collect(),
        published_at: published_at_str,
        created_at: created_at.to_string(),
        updated_at: updated_at.to_string(),
        created_by,
        updated_by,
    })
}

#[derive(derive_new::new, Debug)]
pub struct ContentRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl ContentRepository for ContentRepositoryImpl {
    #[tracing::instrument]
    async fn get(&self, query: GetContentQuery) -> anyhow::Result<Vec<Content>> {
        let rows = sqlx::query_as::<_, ContentsRow>(
            r#"
                SELECT
                    contents.*,
                    category.name AS category_name,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name,
                    (
                        SELECT
                            COALESCE(json_agg(json_build_object('id', tags.id, 'name', tags.name)) 
                            FILTER (WHERE tags.id IS NOT NULL), '[]'::json)
                        FROM
                            tags
                        JOIN
                            content_tags ON content_tags.tag_id = tags.id
                        WHERE
                            content_tags.content_id = contents.id
                    ) AS tags
                FROM
                    contents 
                JOIN
                    category ON contents.category_id = category.id
                JOIN
                    users AS created_by ON created_by.id = contents.created_by
                JOIN
                    users AS updated_by ON updated_by.id = contents.updated_by
               ORDER BY
                    contents.created_at DESC
                LIMIT $1
                OFFSET $2
            "#,
        )
        .bind(query.limit)
        .bind(query.offset)
        .fetch_all(self.db.inner_ref())
        .await?;

        tracing::info!("{:?}", rows);

        rows.into_iter().map(Content::try_from).collect()
    }

    #[tracing::instrument]
    async fn create(&self, data: CreateContent) -> anyhow::Result<Content> {
        let CreateContent {
            title,
            category_id,
            fields,
            status,
            tag_ids,
            created_by_id,
            updated_by_id,
        } = data;

        let uuid = uuid::Uuid::now_v7();
        let category_id = uuid::Uuid::parse_str(&category_id)?;
        let status: ContentRowStatus = status.into();
        let created_by = uuid::Uuid::parse_str(&created_by_id)?;
        let updated_by = uuid::Uuid::parse_str(&updated_by_id)?;
        let tag_uuids: Vec<uuid::Uuid> = tag_ids
            .into_iter()
            .map(|id| uuid::Uuid::from_str(&id))
            .collect::<Result<Vec<_>, _>>()?;

        let content_row = sqlx::query_as::<_, ContentRow>(
            r#"
                WITH inserted AS (
                    INSERT INTO
                        contents (
                            id,
                            title,
                            category_id,
                            fields,
                            status,
                            created_by,
                            updated_by
                        )
                    VALUES ($1, $2, $3, $4, $5, $6, $7)
                    RETURNING *
                )
                SELECT
                    *,
                    category.name AS category_name,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name
                FROM
                    inserted
                JOIN
                    category ON category.id = inserted.category_id 
                JOIN
                    users AS created_by ON created_by.id = inserted.created_by
                JOIN
                    users AS updated_by ON updated_by.id = inserted.updated_by
           "#,
        )
        .bind(uuid)
        .bind(title)
        .bind(category_id)
        .bind(fields)
        .bind(status)
        .bind(created_by)
        .bind(updated_by)
        .fetch_one(self.db.inner_ref())
        .await?;

        tracing::info!("{:?}", content_row);

        let delete_content_tags = sqlx::query(r#"DELETE FROM content_tags WHERE content_id = $1"#)
            .bind(uuid)
            .execute(self.db.inner_ref())
            .await?;

        tracing::info!("{:?}", delete_content_tags);

        let tag_rows = sqlx::query_as::<_, TagRow>(
            r#"
                WITH inserted AS (
                    INSERT INTO
                        content_tags (content_id, tag_id)
                    SELECT
                        $1,
                        tag_id
                    FROM
                        UNNEST($2) AS tag_id
                    RETURNING
                        tag_id
                )
                SELECT 
                    tags.id AS id, tags.name AS name
                FROM
                    inserted
                JOIN
                    tags ON tags.id = inserted.tag_id
            "#,
        )
        .bind(uuid)
        .bind(tag_uuids)
        .fetch_all(self.db.inner_ref())
        .await?;

        tracing::info!("{:?}", tag_rows);

        try_new_content(content_row, tag_rows)
    }

    #[tracing::instrument]
    async fn update(&self, data: UpdateContent) -> anyhow::Result<Content> {
        let UpdateContent {
            id,
            title,
            category_id,
            fields,
            tag_ids,
            status,
            updated_by_id,
        } = data;

        let parsed_content_id = uuid::Uuid::parse_str(&id)?;
        let mut transaction = self.db.inner_ref().begin().await?;

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new(
            "
                WITH updated AS (
                    UPDATE contents SET
            ",
        );
        let mut separated = query_builder.separated(",");

        if let Some(title) = title {
            separated.push("title = ");
            separated.push_bind_unseparated(title);
        }

        if let Some(category_id) = category_id {
            let parsed_category_id = uuid::Uuid::parse_str(&category_id)?;
            separated.push("category_id = ");
            separated.push_bind_unseparated(parsed_category_id);
        }

        if let Some(fields) = fields {
            separated.push("fields = ");
            separated.push_bind_unseparated(fields);
        }

        if let Some(status) = status {
            separated.push("status = ");
            let content_row_status: ContentRowStatus = status.into();
            separated.push_bind_unseparated(content_row_status);
        }

        let parsed_updated_by = uuid::Uuid::parse_str(&updated_by_id)?;
        separated.push("updated_by = ");
        separated.push_bind_unseparated(parsed_updated_by);

        query_builder.push(" WHERE id = ");
        query_builder.push_bind(parsed_content_id);

        query_builder.push(
            "
                    RETURNING *
                )
                SELECT
                    *,
                    category.name AS category_name,
                    created_by.id AS created_by_id,
                    created_by.name AS created_by_name,
                    updated_by.id AS updated_by_id,
                    updated_by.name AS updated_by_name
                FROM
                    updated
                JOIN
                    category ON category.id = updated.category_id
                JOIN
                    users AS created_by ON created_by.id = updated.created_by
                JOIN
                    users AS updated_by ON updated_by.id = updated.updated_by
            ",
        );

        tracing::info!("{:?}", query_builder.sql());

        let content_row = query_builder
            .build_query_as::<ContentRow>()
            .fetch_one(&mut *transaction)
            .await?;

        tracing::info!("{:?}", content_row);

        if let Some(tag_ids) = tag_ids {
            let tag_uuids: Vec<uuid::Uuid> = tag_ids
                .into_iter()
                .map(|id| uuid::Uuid::from_str(&id))
                .collect::<Result<Vec<_>, _>>()?;

            let delete_content_tags =
                sqlx::query(r#"DELETE FROM content_tags WHERE content_id = $1"#)
                    .bind(parsed_content_id)
                    .execute(&mut *transaction)
                    .await?;

            tracing::info!("{:?}", delete_content_tags);

            let insert_content_tags = sqlx::query(
                r#"
                    INSERT INTO
                        content_tags (content_id, tag_id)
                    SELECT
                        $1,
                        tag_id
                    FROM
                        UNNEST($2) AS tag_id
                "#,
            )
            .bind(content_row.id)
            .bind(tag_uuids)
            .execute(&mut *transaction)
            .await?;

            tracing::info!("{:?}", insert_content_tags);
        }

        let tag_rows = sqlx::query_as::<_, TagRow>(
            r#"
                SELECT
                    tags.id AS id, tags.name AS name
                FROM
                    content_tags
                JOIN
                    tags ON tags.id = content_tags.tag_id 
                WHERE
                    content_tags.content_id = $1
            "#,
        )
        .bind(parsed_content_id)
        .fetch_all(&mut *transaction)
        .await?;

        tracing::info!("{:?}", tag_rows);

        try_new_content(content_row, tag_rows)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let parsed_content_id = uuid::Uuid::parse_str(&id)?;

        let result = sqlx::query(r#"DELETE FROM contents WHERE id = $1"#)
            .bind(parsed_content_id)
            .execute(self.db.inner_ref())
            .await?;

        tracing::info!("{:?}", result);

        Ok(())
    }
}
