use std::str::FromStr;

use domain::model::tag::Tag;
use domain::repository::tag::{CreateTag, GetTagQuery, TagRepository, UpdateTag};

use crate::database::connection::ConnectionPool;

#[derive(sqlx::FromRow)]
pub struct TagRow {
    id: uuid::Uuid,
    name: String,
    description: String,
    #[sqlx(skip)]
    #[allow(unused)]
    created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    #[sqlx(skip)]
    #[allow(unused)]
    updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

impl From<TagRow> for Tag {
    fn from(row: TagRow) -> Self {
        let TagRow {
            id,
            name,
            description,
            ..
        } = row;

        Self {
            id: id.into(),
            name,
            description,
        }
    }
}

#[derive(derive_new::new)]
pub struct TagRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl TagRepository for TagRepositoryImpl {
    async fn get(&self, query: GetTagQuery) -> anyhow::Result<Vec<Tag>> {
        let rows =
            sqlx::query_as::<_, TagRow>(r#"SELECT * FROM tags ORDER BY id LIMIT $1 OFFSET $2"#)
                .bind(query.limit)
                .bind(query.offset)
                .fetch_all(self.db.inner_ref())
                .await?;

        Ok(rows.into_iter().map(Tag::from).collect())
    }

    async fn create(&self, tag: CreateTag) -> anyhow::Result<Tag> {
        let CreateTag { name, description } = tag;

        let uuid = uuid::Uuid::now_v7();
        let description = match description {
            Some(str) => str,
            None => "".into(),
        };

        let row = sqlx::query_as::<_, TagRow>(
            r#"INSERT INTO tags (id, name, description) VALUES ($1, $2, $3) RETURNING *"#,
        )
        .bind(uuid)
        .bind(name)
        .bind(description)
        .fetch_one(self.db.inner_ref())
        .await?;

        Ok(Tag::from(row))
    }

    async fn update(&self, tag: UpdateTag) -> anyhow::Result<Tag> {
        let UpdateTag {
            id,
            name,
            description,
        } = tag;

        let mut query_builder =
            sqlx::query_builder::QueryBuilder::<'_, sqlx::Postgres>::new("UPDATE tags SET ");

        let mut separated = query_builder.separated(",");

        if let Some(name) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Some(description) = description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        let uuid = uuid::Uuid::from_str(&id)?;
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(uuid);

        query_builder.push(" RETURNING *");

        let row = query_builder
            .build_query_as::<TagRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        Ok(Tag::from(row))
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let uuid = uuid::Uuid::from_str(&id)?;

        sqlx::query(r#"DELETE FROM tags WHERE id = $1"#)
            .bind(uuid)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
