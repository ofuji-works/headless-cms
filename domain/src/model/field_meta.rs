use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub enum FieldType {
    ShortText,
    LongText,
    RichText,
    Number,
    Boolean,
    DateTime,
    Media,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct FieldMeta {
    pub name: String,
    pub field_id: String,
    pub field_type: FieldType,
    pub is_required: bool,
}

impl FieldMeta {
    pub fn try_new(
        name: String,
        field_id: String,
        field_type: FieldType,
        is_required: bool,
    ) -> Result<Self> {
        if name.len() > 50 {
            bail!("The maximum allowed length is 50 characters.")
        }

        if field_id.len() > 64 {
            bail!("The maximum allowed length is 64 characters.")
        }

        Ok(Self {
            name,
            field_id,
            field_type,
            is_required,
        })
    }
}
