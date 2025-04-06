#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct ContentCategory {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct ContentTag {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub enum ContentStatus {
    Draft,
    Published,
    Reserved,
    Unpublished,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, Clone, derive_new::new)]
pub struct CreatedBy {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, Clone, derive_new::new)]
pub struct UpdatedBy {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct Content {
    #[schema(value_type = String)]
    pub id: uuid::Uuid,
    pub title: String,
    pub category: ContentCategory,
    pub status: ContentStatus,
    pub fields: serde_json::Value,
    pub tags: Vec<ContentTag>,
    pub created_by: CreatedBy,
    pub updated_by: UpdatedBy,
    #[schema(value_type = Option<String>)]
    pub published_at: Option<chrono::DateTime<chrono::Utc>>,
    #[schema(value_type = String)]
    pub created_at: chrono::DateTime<chrono::Utc>,
    #[schema(value_type = String)]
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

impl Content {
    pub fn try_new(
        id: uuid::Uuid,
        title: String,
        category: ContentCategory,
        status: ContentStatus,
        fields: serde_json::Value,
        tags: Vec<ContentTag>,
        created_by: CreatedBy,
        updated_by: UpdatedBy,
        published_at: Option<chrono::DateTime<chrono::Utc>>,
        created_at: chrono::DateTime<chrono::Utc>,
        updated_at: chrono::DateTime<chrono::Utc>,
    ) -> anyhow::Result<Self> {
        if title.len() < 1 {
            anyhow::bail!("Title must be at least 1 character long")
        }

        if title.len() > 50 {
            anyhow::bail!("Title exceeds maximum length(50 characters)")
        }

        Ok(Self {
            id,
            title,
            category,
            status,
            fields,
            tags,
            created_by,
            updated_by,
            published_at,
            created_at,
            updated_at,
        })
    }
}
