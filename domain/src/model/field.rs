use derive_new::new;

use crate::model::field_meta::FieldMeta;

#[derive(new)]
pub struct Field {
    pub meta: FieldMeta,
    pub value: String,
}
