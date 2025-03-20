#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, Clone)]
pub struct Category {
    pub id: String,
    pub name: String,
    pub api_identifier: String,
    pub description: Option<String>,
}

impl Category {
    pub fn try_new(
        id: String,
        name: String,
        api_identifier: String,
        description: Option<String>,
    ) -> anyhow::Result<Self> {
        if name.len() < 1 {
            anyhow::bail!("The minimum allowed length is 1 characters.");
        }

        if name.len() > 50 {
            anyhow::bail!("The maximum allowed length is 50 characters.");
        }

        if api_identifier.len() < 1 {
            anyhow::bail!("The minimum allowed length is 1 characters.");
        }

        if api_identifier.len() > 64 {
            anyhow::bail!("The maximum allowed length is 64 characters.");
        }

        if let Some(desc) = &description {
            if desc.len() > 500 {
                anyhow::bail!("The maximum allowed length is 500 characters.");
            }
        }

        Ok(Self {
            id,
            name,
            api_identifier,
            description,
        })
    }
}
