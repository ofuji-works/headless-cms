use crate::field_meta::FieldMeta;

pub struct Field {
    pub meta: FieldMeta,
    pub value: String,
}

impl Field {
    pub fn new(meta: FieldMeta, value: String) -> Self {
        Self {
            meta,
            value,
        }
    }
}
