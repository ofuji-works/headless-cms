use derive_new::new;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::category::Category;

#[derive(Debug, Deserialize, Serialize, ToSchema)]
pub enum FieldType {
    ShortText,
    LongText,
    RichText,
    Boolean,
    Media,
}

#[derive(Debug, Deserialize, Serialize, new, ToSchema)]
pub struct Field {
    pub field_type: FieldType,
    pub value: String,
}

#[derive(Debug, Deserialize, Serialize, new, ToSchema)]
pub enum ContentStatus {
    Draft,
    Published,
    Reserved,
    Unpublished,
}

#[derive(Debug, Deserialize, Serialize, new, ToSchema)]
pub struct Content {
    pub id: String,
    pub category: Category,
    pub status: ContentStatus,
    pub fields: Vec<Field>,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
