#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub enum FieldType {
    ShortText,
    LongText,
    RichText,
    Boolean,
    Media,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct Field {
    pub field_type: FieldType,
    pub value: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct Categories {
    pub id: String,
    pub name: String,
}

#[derive(Debug, serde::Deserialize, serde::Serialize, derive_new::new, utoipa::ToSchema)]
pub struct Tags {
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
    pub id: String,
    pub title: String,
    pub categories: Categories,
    pub status: ContentStatus,
    pub fields: Vec<Field>,
    pub tags: Vec<Tags>,
    pub published_at: Option<String>,
    pub created_by: CreatedBy,
    pub updated_by: UpdatedBy,
    pub created_at: String,
    pub updated_at: String,
}
