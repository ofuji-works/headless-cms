use anyhow::{bail, Error, Result};
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
        content::{Content, ContentStatus, Field},
        content_model::ContentModel,
    },
    repository::content::{ContentRepository, CreateContent, UpdateContent},
};

use crate::database::ConnectionPool;

#[derive(Debug, FromRow)]
pub struct ContentRow {
    pub content_id: Uuid,
    pub fields: Value,
    pub status: Value,
    pub published_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub content_model_id: Uuid,
    pub content_model_name: String,
    pub content_model_api_identifier: String,
    pub content_model_description: Option<String>,
}

impl TryFrom<ContentRow> for Content {
    type Error = Error;
    fn try_from(row: ContentRow) -> Result<Self> {
        let ContentRow {
            content_id,
            fields,
            status,
            published_at,
            created_at,
            updated_at,
            content_model_id,
            content_model_name,
            content_model_api_identifier,
            content_model_description,
        } = row;

        let content_model = ContentModel::try_new(
            content_model_id.into(),
            content_model_name,
            content_model_api_identifier,
            content_model_description,
        )?;
        let deserialized_status: ContentStatus = serde_json::from_value(status)?;
        let deserialized_fields: Vec<Field> = serde_json::from_value(fields)?;
        let published_at_str: Option<String> = match published_at {
            Some(datetime) => Some(datetime.to_string()),
            None => None,
        };

        Ok(Self {
            id: content_id.to_string(),
            model: content_model,
            status: deserialized_status,
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
        let rows: Vec<ContentRow> = sqlx::query_as!(
            ContentRow,
            r#"
                SELECT
                    c.content_id,
                    c.fields,
                    c.status,
                    c.published_at,
                    c.created_at,
                    c.updated_at,
                    m.content_model_id,
                    m.name AS content_model_name,
                    m.api_identifier AS content_model_api_identifier,
                    m.description AS content_model_description
                FROM contents c 
                INNER JOIN content_model m
                ON c.content_model_id = m.content_model_id
            "#
        )
        .fetch_all(self.db.inner_ref())
        .await?;

        rows.into_iter().map(Content::try_from).collect()
    }

    async fn create(&self, data: CreateContent) -> Result<()> {
        let content_model_id = Uuid::parse_str(&data.content_model_id)?;
        let status = serde_json::to_string(&data.status)?;

        sqlx::query!(
            r#"INSERT INTO contents (content_model_id, fields, status) VALUES ($1, $2, $3)"#,
            content_model_id,
            data.fields,
            status,
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }

    async fn update(&self, data: UpdateContent) -> Result<()> {
        let UpdateContent {
            id, fields, status, ..
        } = data;

        let parsed_content_id = Uuid::parse_str(&id)?;

        let mut set_params: Vec<String> = Vec::new();

        if let Some(fields) = fields {
            set_params.push(format!("fields = {}", serde_json::to_value(fields)?));
        }

        if let Some(status) = status {
            set_params.push(format!("status = {}", serde_json::to_string(&status)?));
        }

        if set_params.len() < 1 {
            bail!("")
        }

        let update_params_str = set_params.join(",");

        sqlx::query(r#"UPDATE contents SET $1 WHERE content_id = $2"#)
            .bind(update_params_str)
            .bind(parsed_content_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }

    async fn delete(&self, id: String) -> Result<()> {
        let parsed_content_id = Uuid::parse_str(&id)?;

        sqlx::query!(
            r#"DELETE FROM contents WHERE content_id = $1"#,
            parsed_content_id
        )
        .execute(self.db.inner_ref())
        .await?;

        Ok(())
    }
}
