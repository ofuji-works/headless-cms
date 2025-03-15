use domain::model::{role::Role, user::User};
use domain::repository::user::{CreateUser, GetUserQuery, UpdateUser, UserRepository};

use crate::database::connection::ConnectionPool;

#[derive(sqlx::FromRow)]
pub struct UserRow {
    id: uuid::Uuid,
    name: String,
    icon_url: String,
    role_id: uuid::Uuid,
    role_name: String,
    role_description: Option<String>,
    role_is_super_administrator: bool,
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
            id,
            name,
            icon_url,
            role_id,
            role_name,
            role_description,
            role_is_super_administrator,
            ..
        } = row;

        let role = Role::try_new(
            role_id.into(),
            role_name,
            role_description,
            role_is_super_administrator,
        )?;

        Ok(Self {
            id: id.to_string(),
            name,
            icon_url,
            role,
        })
    }
}

#[derive(derive_new::new)]
pub struct UserRepositoryImpl {
    db: ConnectionPool,
}

#[async_trait::async_trait]
impl UserRepository for UserRepositoryImpl {
    async fn get(&self, query: GetUserQuery) -> anyhow::Result<Vec<User>> {
        let rows = sqlx::query_as::<_, UserRow>(
            r#"
                SELECT
                    users.*,
                    role.name AS role_name,
                    role.description AS role_description,
                    role.is_super_administrator AS role_is_super_administrator
                FROM users
                JOIN
                    role ON users.role_id = role.id
                LIMIT $1
                OFFSET $2
            "#,
        )
        .bind(query.limit)
        .bind(query.offset)
        .fetch_all(self.db.inner_ref())
        .await?;

        rows.into_iter().map(User::try_from).collect()
    }

    async fn find(&self, id: String) -> anyhow::Result<User> {
        let row = sqlx::query_as::<_, UserRow>(
            r#"
                SELECT
                    users.*,
                    role.name AS role_name,
                    role.description AS role_description,
                    role.is_super_administrator AS role_is_super_administrator
                FROM users
                JOIN
                    role ON users.role_id = role.id
                WHERE user.id = $1
            "#,
        )
        .bind(id)
        .fetch_one(self.db.inner_ref())
        .await?;

        User::try_from(row)
    }

    async fn create(&self, create_user: CreateUser) -> anyhow::Result<User> {
        let CreateUser {
            name,
            icon_url,
            role_id,
        } = create_user;

        let uuid = uuid::Uuid::now_v7();

        let row = sqlx::query_as::<_, UserRow>(
            r#"
                WITH inserted AS (
                    INSERT INTO
                        users (
                            id,
                            name,
                            icon_url,
                            role_id,
                        )
                    VALUES
                        ($1, $2, $3, $4)
                    RETURNING
                        users.*
                )
                SELECT
                    inserted.*,
                    role.name AS role_name,
                    role.description AS role_description,
                    role.is_super_administrator AS role_is_super_administrator
                FROM
                    inserted
                JOIN
                    role ON inserted.role_id = role.id
            "#,
        )
        .bind(uuid)
        .bind(name)
        .bind(icon_url)
        .bind(role_id)
        .fetch_one(self.db.inner_ref())
        .await?;

        User::try_from(row)
    }

    async fn update(&self, update_user: UpdateUser) -> anyhow::Result<User> {
        let UpdateUser {
            id,
            name,
            icon_url,
            role_id,
        } = update_user;

        let mut query_builder = sqlx::QueryBuilder::<'_, sqlx::Postgres>::new(
            "
                WITH updated AS (
                    UPDATE users SET
            ",
        );
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
            separated.push("role_id = ");
            separated.push_bind_unseparated(role_id);
        }

        let parsed_user_id = uuid::Uuid::parse_str(&id)?;
        query_builder.push(" WHERE users.id = ");
        query_builder.push_bind(parsed_user_id);

        query_builder.push(
            "
                    RETURNING *
                )
                SELECT
                    updated.*,
                    role.name AS role_name,
                    role.description AS role_description,
                    role.is_super_administrator AS role_is_super_administrator
                FROM
                    updated
                JOIN
                    role ON role.id = updated.role_id
            ",
        );

        let row = query_builder
            .build_query_as::<UserRow>()
            .fetch_one(self.db.inner_ref())
            .await?;

        User::try_from(row)
    }

    async fn delete(&self, id: String) -> anyhow::Result<()> {
        let parsed_id = uuid::Uuid::parse_str(&id)?;

        sqlx::query(r#"DELETE FROM users id = $1"#)
            .bind(parsed_id)
            .execute(self.db.inner_ref())
            .await?;

        Ok(())
    }
}
