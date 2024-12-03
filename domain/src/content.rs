use crate::content_model::ContentModel;
use crate::field::Field;

pub struct Content {
    pub model:  ContentModel,
    pub fields: Vec<Field>,
}

impl Content {
    pub fn new(model: ContentModel, fields: Vec<Field>) -> Self {
        Self { model, fields }
    }
}

