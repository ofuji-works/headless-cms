use derive_new::new;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::model::content_model::ContentModel;
use crate::model::field::Field;

#[derive(Debug, Deserialize, Serialize, new, ToSchema)]
pub struct Content {
    pub id: String,
    pub model: ContentModel,
    pub is_draft: bool,
    pub field_values: Vec<Field>,
    pub published_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
}
