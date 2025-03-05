#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema)]
pub struct Tag {
    pub id: String,
    pub name: String,
    pub description: String,
}

impl Tag {
    pub fn try_new(id: String, name: String, description: String) -> anyhow::Result<Self> {
        if name.len() > 50 {
            anyhow::bail!("The maximum allowed length is 50 characters.");
        }

        if description.len() > 500 {
            anyhow::bail!("The maximum allowed length is 500 characters.");
        }

        Ok(Self {
            id,
            name,
            description,
        })
    }
}
