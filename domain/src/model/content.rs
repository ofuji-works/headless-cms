use derive_new::new;

use crate::model::content_model::ContentModel;
use crate::model::field::Field;

#[derive(Debug, new)]
pub struct Content {
    pub model: ContentModel,
    pub fields: Vec<Field>,
}
