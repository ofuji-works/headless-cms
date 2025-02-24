#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, Clone)]
pub struct Role {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub is_super_administrator: bool,
}

impl Role {
    pub fn try_new(
        id: String,
        name: String,
        description: Option<String>,
        is_super_administrator: bool,
    ) -> anyhow::Result<Self> {
        if name.len() > 50 {
            anyhow::bail!("The maximum allowed length is 50 characters.");
        }

        if let Some(desc) = &description {
            if desc.len() > 500 {
                anyhow::bail!("The maximum allowed length is 50 characters.");
            }
        }

        Ok(Self {
            id,
            name,
            description,
            is_super_administrator,
        })
    }
}
