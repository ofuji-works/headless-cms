use std::str::FromStr;

use anyhow::{bail, Error, Result};
use async_trait::async_trait;
use derive_new::new;
use domain::{
    model::content_model::ContentModel,
    repository::content_model::{ContentModelRepository, CreateContentModel, UpdateContentModel},
};
use sqlx::{
    types::{
        chrono::{DateTime, Utc},
        Uuid,
    },
    FromRow,
};

use crate::database::ConnectionPool;

#[derive(Debug, FromRow)]
struct ContentModelRow {
    pub content_model_id: Uuid,
    pub name: String,
    pub api_identifier: String,
    pub description: String,
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
            ..
        } = row;

        Ok(Self {
            id: content_model_id.into(),
            name,
            api_identifier,
            description: Some(description),
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
        let rows: Vec<ContentModelRow> =
            sqlx::query_as::<_, ContentModelRow>(r#"SELECT * FROM content_model"#)
                .fetch_all(self.db.inner_ref())
                .await?;

        rows.into_iter().map(ContentModel::try_from).collect()
    }

    async fn create(&self, data: CreateContentModel) -> Result<ContentModel> {
        let description = match data.description {
            Some(str) => str,
            None => "".into(),
        };

        let content_model_row = sqlx::query_as::<_, ContentModelRow>(
            r#"
                INSERT INTO
                    content_model (
                        name,
                        api_identifier,
                        description
                    )
                VALUES
                    ($1, $2, $3)
                RETURNING
                    content_model_id,
                    name,
                    api_identifier,
                    description,
                    created_at,
                    updated_at
            "#,
        )
        .bind(data.name)
        .bind(data.api_identifier)
        .bind(description)
        .fetch_one(self.db.inner_ref())
        .await?;

        ContentModel::try_from(content_model_row)
    }

    async fn update(&self, data: UpdateContentModel) -> Result<ContentModel> {
        let UpdateContentModel {
            id,
            name,
            api_identifier,
            description,
        } = data;

        let content_model_id = Uuid::from_str(&id)?;

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

        if set_params.len() < 1 {
            bail!("")
        }

        let update_params_str = set_params.join(",");

        let content_model_row = sqlx::query_as::<_, ContentModelRow>(
            r#"
                UPDATE
                    content_model
                SET
                    $1
                WHERE
                    content_model_id = $2
                RETURNING
                    content_model_id,
                    name,
                    api_identifier,
                    description,
                    created_at,
                    updated_at
            "#,
        )
        .bind(update_params_str)
        .bind(content_model_id)
        .fetch_one(self.db.inner_ref())
        .await?;

        ContentModel::try_from(content_model_row)
    }

    async fn delete(&self, id: String) -> Result<()> {
        let content_model_id = Uuid::from_str(&id)?;

        sqlx::query(r#"DELETE FROM content_model WHERE content_model_id = $1"#)
            .bind(content_model_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
