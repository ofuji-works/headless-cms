use derive_new::new;
use serde::{Deserialize, Serialize};

use crate::model::field_meta::FieldMeta;

#[derive(Debug, Deserialize, Serialize ,new)]
pub struct Field {
    pub meta: FieldMeta,
    pub value: String,
}
