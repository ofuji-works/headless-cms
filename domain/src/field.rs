use anyhow::{bail, Result};

pub struct Field {
    name: String,
    field_id: String,
    is_required: bool,
}

impl Field {
    pub fn try_new (name: String, field_id: String, is_required: bool) -> Result<Self> {

        if name.len() > 50 {
            bail!("The maximum allowed length is 50 characters.")
        }

        if field_id.len() > 64 {
            bail!("The maximum allowed length is 64 characters.")
        }

        Ok(Self {
            name,
            field_id,
            is_required,
        })
    }
}
