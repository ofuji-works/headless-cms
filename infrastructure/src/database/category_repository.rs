use sqlx::types::chrono::{DateTime, Utc};
use sqlx::types::Uuid;
use std::str::FromStr;

use domain::model::category::Category;
use domain::repository::category::{CategoryRepository, CreateCategory, UpdateCategory};

use crate::database::connection::ConnectionPool;

#[derive(Debug, sqlx::FromRow)]
struct CategoryRow {
    pub id: Uuid,
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

impl TryFrom<CategoryRow> for Category {
    type Error = anyhow::Error;

    fn try_from(row: CategoryRow) -> anyhow::Result<Self> {
        let CategoryRow {
            id,
            name,
            api_identifier,
            description,
            ..
        } = row;

        Ok(Self {
            id: id.into(),
            name,
            api_identifier,
            description: Some(description),
        })
    }
}

#[derive(derive_new::new)]
pub struct CategoryRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl CategoryRepository for CategoryRepositoryImpl {
    async fn get(&self) -> anyhow::Result<Vec<Category>> {
        let rows: Vec<CategoryRow> = sqlx::query_as::<_, CategoryRow>(r#"SELECT * FROM category"#)
            .fetch_all(self.db.inner_ref())
            .await?;

        rows.into_iter().map(Category::try_from).collect()
    }

    async fn create(&self, data: CreateCategory) -> anyhow::Result<Category> {
        let CreateCategory {
            name,
            api_identifier,
            description,
        } = data;

        let description = match description {
            Some(str) => str,
            None => "".into(),
        };

        let category_row = sqlx::query_as::<_, CategoryRow>(
            r#"
                INSERT INTO
                    category (
                        name,
                        api_identifier,
                        description
                    )
                VALUES ($1, $2, $3)
                RETURNING
                    *
            "#,
        )
        .bind(name)
        .bind(api_identifier)
        .bind(description)
        .fetch_one(self.db.inner_ref())
        .await?;

        Category::try_from(category_row)
    }

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

        let category_id = Uuid::from_str(&id)?;
        query_builder.push(" WHERE id = ");
        query_builder.push_bind(category_id);

        query_builder.push(" RETURNING *");

        let category_row = query_builder
            .build_query_as::<CategoryRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        Category::try_from(category_row)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let category_id = Uuid::from_str(&id)?;

        sqlx::query(r#"DELETE FROM category WHERE id = $1"#)
            .bind(category_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
