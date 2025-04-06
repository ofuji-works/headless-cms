use domain::model::user::{Admin, Member, User};
use domain::repository::user::{CreateUser, GetUserQuery, UpdateUser, UserRepository};

use crate::database::connection::ConnectionPool;

enum Role {
    Admin,
    Memeber,
}

#[derive(sqlx::FromRow, Debug)]
pub struct UserRow {
    id: uuid::Uuid,
    name: String,
    icon_url: String,
    role: Role,
    #[sqlx(skip)]
    #[allow(unused)]
    deleted_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    #[sqlx(skip)]
    #[allow(unused)]
    created_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
    #[sqlx(skip)]
    #[allow(unused)]
    updated_at: sqlx::types::chrono::DateTime<sqlx::types::chrono::Utc>,
}

impl TryFrom<UserRow> for User {
    type Error = anyhow::Error;

    fn try_from(row: UserRow) -> anyhow::Result<Self> {
        let UserRow {
            id, name, icon_url, ..
        } = row;

        Ok(Self {
            id: id.to_string(),
            name,
            icon_url,
            _role: std::marker::PhantomData,
        })
    }
}

#[derive(derive_new::new, Debug)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    #[tracing::instrument]
    async fn get(&self, query: GetUserQuery) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users LIMIT $1 OFFSET $2"#)
            .bind(query.limit)
            .bind(query.offset)
            .fetch_all(self.db.inner_ref())
            .await?;
        let result = rows.into_iter().map(User::try_from).collect();
        tracing::info!("{:?}", result);

        result
    }

    #[tracing::instrument]
    async fn find(&self, id: String) -> anyhow::Result<User> {
        let row = sqlx::query_as::<_, UserRow>(r#"SELECT * FROM users WHERE user.id = $1"#)
            .bind(id)
            .fetch_one(self.db.inner_ref())
            .await?;
        let result = User::try_from(row);
        tracing::info!("{:?}", result);

        result
    }

    #[tracing::instrument]
    async fn create(&self, create_user: CreateUser) -> anyhow::Result<User> {
        let CreateUser {
            name,
            icon_url,
            role_id,
        } = create_user;
        let uuid = uuid::Uuid::now_v7();
        let row = sqlx::query_as::<_, UserRow>(
            r#"INSERT INTO users (id, name, icon_url, role) VALUES ($1, $2, $3, $4) RETURNING *"#,
        )
        .bind(uuid)
        .bind(name)
        .bind(icon_url)
        .bind(role)
        .fetch_one(self.db.inner_ref())
        .await?;
        let result = User::try_from(row);
        tracing::info!("{:?}", result);

        result
    }

    #[tracing::instrument]
    async fn update(&self, update_user: UpdateUser) -> anyhow::Result<User> {
        let UpdateUser {
            id,
            name,
            icon_url,
            role_id,
        } = update_user;

        let mut query_builder = sqlx::QueryBuilder::<'_, sqlx::Postgres>::new("UPDATE users SET");
        let mut separated = query_builder.separated(",");

        if let Some(name) = name {
            separated.push("name = ");
            separated.push_bind_unseparated(name);
        }

        if let Some(icon_url) = icon_url {
            separated.push("icon_url = ");
            separated.push_bind_unseparated(icon_url);
        }

        if let Some(role_id) = role_id {
            separated.push("role = ");
            separated.push_bind_unseparated(role_id);
        }

        let parsed_user_id = uuid::Uuid::parse_str(&id)?;
        query_builder.push(" WHERE users.id = ");
        query_builder.push_bind(parsed_user_id);
        query_builder.push(" RETURNING *");

        let row = query_builder
            .build_query_as::<UserRow>()
            .fetch_one(self.db.inner_ref())
            .await?;
        let result = User::try_from(row);
        tracing::info!("{:?}", result);

        result
    }

    #[tracing::instrument]
    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let parsed_id = uuid::Uuid::parse_str(&id)?;

        let result = sqlx::query(r#"DELETE FROM users id = $1"#)
            .bind(parsed_id)
            .execute(self.db.inner_ref())
            .await?;

        tracing::info!("{:?}", result);

        Ok(())
    }
}
