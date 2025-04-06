use crate::model::user::{Role, User};

#[derive(derive_new::new, Debug)]
pub struct GetUserQuery {
    pub limit: i32,
    pub offset: i32,
}

impl Default for GetUserQuery {
    fn default() -> Self {
        Self {
            limit: 100,
            offset: 0,
        }
    }
}

#[derive(derive_new::new, Debug)]
pub struct CreateUser {
    pub name: String,
    pub icon_url: String,
    pub role_id: String,
}

#[derive(derive_new::new, Debug)]
pub struct UpdateUser {
    pub id: String,
    pub name: Option<String>,
    pub icon_url: Option<String>,
    pub role_id: Option<String>,
}

#[async_trait::async_trait]
pub trait UserRepository<T: Role>: Send + Sync {
    async fn get(&self, query: GetUserQuery) -> anyhow::Result<Vec<User<T>>>;
    async fn find(&self, id: String) -> anyhow::Result<User<T>>;
    async fn create(&self, create_user: CreateUser) -> anyhow::Result<User<T>>;
    async fn update(&self, update_user: UpdateUser) -> anyhow::Result<User<T>>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
