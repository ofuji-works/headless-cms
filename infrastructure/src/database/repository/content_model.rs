use anyhow::Result;
use async_trait::async_trait;
use derive_new::new;
use domain::{
    model::{content_model::ContentModel, field_meta::FieldMeta},
    repository::content_model::ContentModelRepository,
};

use crate::database::ConnectionPool;

struct ContentModelRow {
    name: String,
    api_identifier: String,
}

impl From<ContentModelRow> for ContentModel {
    fn from(row: ContentModelRow) -> Self {
        let fields: Vec<FieldMeta> = vec![];

        Self {
            name: row.name,
            api_identifier: row.api_identifier,
            description: Some("".into()),
            fields,
        }
    }
}

#[derive(new)]
pub struct ContentModelRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait]
impl ContentModelRepository for ContentModelRepositoryImpl {
    async fn get(&self) -> Result<Vec<ContentModel>> {
        // let rows: Vec<ContentModelRow> = sqlx::query_as!(ContentModelRow, r#"SELECT 1 from m_content_model"#)
            //.fetch_all(self.db.inner_ref())
            // .await?;

        let vec: Vec<ContentModel> = vec![];
        Ok(vec)
        // Ok(rows.into_iter().map(ContentModel::from).collect())
    }
}
