use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, derive_new::new)]
pub struct CreatedBy {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone, derive_new::new)]
pub struct UpdatedBy {
    id: String,
    name: String,
}

#[derive(Debug, Deserialize, Serialize, ToSchema, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
    pub created_by: CreatedBy,
    pub updated_by: UpdatedBy,
}

impl Category {
    pub fn try_new(
        id: String,
        name: String,
        api_identifier: String,
        description: Option<String>,
        created_by: CreatedBy,
        updated_by: UpdatedBy,
    ) -> Result<Self> {
        if name.len() > 50 {
            bail!("The maximum allowed length is 50 characters.");
        }

        if api_identifier.len() > 64 {
            bail!("The maximum allowed length is 64 characters.");
        }

        if let Some(desc) = &description {
            if desc.len() > 500 {
                bail!("The maximum allowed length is 50 characters.");
            }
        }

        Ok(Self {
            id,
            name,
            api_identifier,
            description,
            created_by,
            updated_by,
        })
    }
}
