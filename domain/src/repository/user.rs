use crate::model::user::User;

#[derive(derive_new::new)]
pub struct GetUserQuery {
    pub offset: i32,
    pub limit: i32,
}

impl Default for GetUserQuery {
    fn default() -> Self {
        Self {
            offset: 0,
            limit: 100,
        }
    }
}

pub struct CreateUser {
    pub name: String,
    pub icon_url: String,
    pub role_id: String,
}

pub struct UpdateUser {
    pub id: String,
    pub name: Option<String>,
    pub icon_url: Option<String>,
    pub role_id: Option<String>,
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get(&self, query: GetUserQuery) -> anyhow::Result<Vec<User>>;
    async fn find(&self, id: String) -> anyhow::Result<User>;
    async fn create(&self, create_user: CreateUser) -> anyhow::Result<User>;
    async fn update(&self, update_user: UpdateUser) -> anyhow::Result<User>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
