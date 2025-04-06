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
            anyhow::bail!("Name must be at least 1 character long");
        }

        if name.len() > 50 {
            anyhow::bail!("Name exceeds maximum length(50 characters)");
        }

        if api_identifier.len() < 1 {
            anyhow::bail!("Api identifier must be at least 1 character long");
        }

        if api_identifier.len() > 64 {
            anyhow::bail!("Api identifier exceeds maximum length(64 characters)");
        }

        if let Some(desc) = &description {
            if desc.len() > 500 {
                anyhow::bail!("Description exceeds maximum length(500 characters)");
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
