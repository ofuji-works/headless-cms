use std::str::FromStr;

use domain::model::category::Category;
use domain::repository::category::{
    CategoryRepository, CreateCategory, GetCategoryQuery, UpdateCategory,
};

use crate::database::connection::ConnectionPool;

#[derive(Debug, sqlx::FromRow)]
struct CategoryRow {
    pub id: uuid::Uuid,
    pub name: String,
    pub api_identifier: String,
    pub description: String,
    #[sqlx(skip)]
    #[allow(unused)]
    pub created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    #[sqlx(skip)]
    #[allow(unused)]
    pub updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

impl From<CategoryRow> for Category {
    fn from(row: CategoryRow) -> Self {
        let CategoryRow {
            id,
            name,
            api_identifier,
            description,
            ..
        } = row;

        Self {
            id: id.into(),
            name,
            api_identifier,
            description: Some(description),
        }
    }
}

#[derive(derive_new::new, Debug)]
pub struct CategoryRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    #[tracing::instrument]
    async fn get(&self, query: GetCategoryQuery) -> anyhow::Result<Vec<Category>> {
        let rows: Vec<CategoryRow> = sqlx::query_as::<_, CategoryRow>(
            r#"SELECT * FROM category ORDER BY id LIMIT $1 OFFSET $2"#,
        )
        .bind(query.limit)
        .bind(query.offset)
        .fetch_all(self.db.inner_ref())
        .await?;

        tracing::info!("{:?}", rows);

        Ok(rows.into_iter().map(Category::from).collect())
    }

    #[tracing::instrument]
    async fn create(&self, data: CreateCategory) -> anyhow::Result<Category> {
        let CreateCategory {
            name,
            api_identifier,
            description,
        } = data;

        let id = uuid::Uuid::now_v7();

        let description = match description {
            Some(str) => str,
            None => "".into(),
        };

        let category_row = sqlx::query_as::<_, CategoryRow>(
            r#"
                INSERT INTO
                    category (
                        id,
                        name,
                        api_identifier,
                        description
                    )
                VALUES ($1, $2, $3, $4)
                RETURNING *
            "#,
        )
        .bind(id)
        .bind(name)
        .bind(api_identifier)
        .bind(description)
        .fetch_one(self.db.inner_ref())
        .await?;

        tracing::info!("{:?}", category_row);

        Ok(Category::from(category_row))
    }

    #[tracing::instrument]
    async fn update(&self, data: UpdateCategory) -> anyhow::Result<Category> {
        let UpdateCategory {
            id,
            name,
            api_identifier,
            description,
        } = data;

        let mut query_builder = sqlx::QueryBuilder::<sqlx::Postgres>::new("UPDATE category SET ");
        let mut separated = query_builder.separated(",");

        if let Some(name) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Some(api_identifier) = api_identifier {
            separated.push("api_identifier = ");
            separated.push_bind_unseparated(api_identifier);
        }

        if let Some(description) = description {
            separated.push("description = ");
            separated.push_bind_unseparated(description);
        }

        let category_id = uuid::Uuid::from_str(&id)?;
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(category_id);

        query_builder.push(" RETURNING *");

        tracing::info!("{:?}", query_builder.sql());

        let category_row = query_builder
            .build_query_as::<CategoryRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        tracing::info!("{:?}", category_row);

        Ok(Category::from(category_row))
    }

    #[tracing::instrument]
    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let category_id = uuid::Uuid::from_str(&id)?;

        let result = sqlx::query(r#"DELETE FROM category WHERE id = $1"#)
            .bind(category_id)
            .execute(self.db.inner_ref())
            .await?;

        tracing::info!("{:?}", result);

        Ok(())
    }
}
