#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn try_new(id: String, name: String, description: String) -> anyhow::Result<Self> {
        if name.len() < 1 {
            anyhow::bail!("Name must be at least 1 character long");
        }

        if name.len() > 50 {
            anyhow::bail!("Name exceeds maximum length(50 characters)");
        }

        if description.len() > 500 {
            anyhow::bail!("Description exceeds maximum length(50 characters)");
        }

        Ok(Self {
            id,
            name,
            description,
        })
    }
}
