use anyhow::{Result, Error};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    model::{content_model::ContentModel, field_meta::FieldMeta},
    repository::content_model::{ContentModelRepository, CreateContentModel, UpdateContentModel}
};
use sqlx::{types::{chrono::{DateTime, Utc}, Uuid}, FromRow};

use crate::database::ConnectionPool;

#[derive(Debug, FromRow)]
struct ContentModelRow {
    pub content_model_id: Uuid,
    pub name: String,
    pub api_identifier: String,
    pub description: String,
    pub fields: serde_json::Value,
    #[sqlx(skip)]
    #[allow(unused)]
    pub created_at: DateTime<Utc>,
    #[sqlx(skip)]
    #[allow(unused)]
    pub updated_at: DateTime<Utc>,
}

impl TryFrom<ContentModelRow> for ContentModel {
    type Error = Error;
    fn try_from(row: ContentModelRow) -> Result<Self> {
        let ContentModelRow {
            content_model_id,
            name,
            api_identifier,
            description,
            fields,
            ..
        } = row;
        let fields: Vec<FieldMeta> = serde_json::from_value(fields)?;

        Ok(Self {
            id: content_model_id.into(),
            name,
            api_identifier,
            description: Some(description),
            fields,
        })
    }
}

#[derive(new)]
pub struct ContentModelRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ContentModelRepository for ContentModelRepositoryImpl {
    async fn get(&self) -> Result<Vec<ContentModel>> {
        let rows: Vec<ContentModelRow> = sqlx::query_as!(
                ContentModelRow,
                r#"SELECT * from content_model"#
            )
            .fetch_all(self.db.inner_ref())
            .await?;

        rows.into_iter().map(ContentModel::try_from).collect()
    }

    async fn create(&self, data: CreateContentModel) -> Result<()> {

        let description = data.description.or_else("".into());
        let fields = serde_json::to_value(data.fields)?;

        sqlx::query!(r#"
            INSERT INTO content_model (name, api_identifier, description, fields) VALUES ($1, $2, $3, $4)
        "#,
            data.name,
            data.api_identifier,
            description,
            fields,
        ).excute(self.db.inner_ref()).await
    }

    async fn update(&self, data: UpdateContentModel) -> Result<()> {

        let UpdateContentModel {
            id,
            name,
            api_identifier,
            description,
            fields,
        } = data;

        let mut set_params: Vec<String> = Vec::new();

        if let Some(name) = name {
            set_params.push(format!("name = {}", name));
        }

        if let Some(api_identifier) = api_identifier {
            set_params.push(format!("api_identifier = {}", api_identifier));
        }

        if let Some(description) = description {
            set_params.push(format!("description = {}", description));
        }

        if let Some(fields) = fields {
            set_params.push(format!("fields = {}", serde_json::to_value(fields)?));
        }

        sqlx::query!(
            "UPDATE content_model SET {} WHERE content_model_id = {}",
            set_params.join(","),
            id,
        ).excute(self.db.inner_ref()).await
    }

    async fn delete(&self, id: String) -> Result<()> {
        sqlx::query!(
            r#"DELETE FROM content_model WHERE content_model_id = $1"#,
            id,
        ).excute(self.db.inner_ref()).await
    }
}
