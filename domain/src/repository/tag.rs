use crate::model::tag::Tag;

pub struct GetTagQuery {
    pub offset: i32,
    pub limit: i32,
}

pub struct CreateTag {
    pub name: String,
    pub description: Option<String>,
}

pub struct UpdateTag {
    pub id: String,
    pub name: Option<String>,
    pub description: Option<String>,
}

#[async_trait::async_trait]
pub trait TagRepository: Send + Sync {
    async fn get(&self, query: GetTagQuery) -> anyhow::Result<Vec<Tag>>;
    async fn create(&self, create_tag: CreateTag) -> anyhow::Result<Tag>;
    async fn update(&self, update_tag: UpdateTag) -> anyhow::Result<Tag>;
    async fn delete(&self, id: String) -> anyhow::Result<()>;
}
