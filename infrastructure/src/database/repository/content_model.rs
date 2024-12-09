use anyhow::{Result, Error};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    model::{content_model::ContentModel, field_meta::FieldMeta},
    repository::content_model::{CreateContentModel, ContentModelRepository}
};
use sqlx::{types::{chrono::{DateTime, Utc}, Uuid}, FromRow};

use crate::database::ConnectionPool;

#[derive(Debug, FromRow)]
struct ContentModelRow {
    #[sqlx(skip)]
    #[allow(unused)]
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
            name,
            api_identifier,
            description,
            fields,
            ..
        } = row;
        let fields: Vec<FieldMeta> = serde_json::from_value(fields)?;

        Ok(Self {
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

    async fn create(&self, content_model: CreateContentModel) -> Result<()> {

        let description = content_model.description.or_else("".into());
        let fields = serde_json::to_value(content_model.fields)?;

        sqlx::query!(r#"
            INSERT INTO content_model (name, api_identifier, description, fields) VALUES ($1, $2, $3, $4)
        "#,
            content_model.name,
            content_model.api_identifier,
            description,
            fields,
        ).excute(self.db.inner_ref()).await?;

        Ok(())
    }
}
