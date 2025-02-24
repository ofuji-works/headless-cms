use crate::model::role::Role;

#[derive(Debug, serde::Deserialize, serde::Serialize, utoipa::ToSchema, Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    pub role: Role,
}

impl User {
    pub fn try_new(id: String, name: String, icon_url: String, role: Role) -> anyhow::Result<Self> {
        if name.len() > 50 {
            anyhow::bail!("The maximum allowed length is 50 characters.");
        }

        if url::Url::parse(&icon_url).is_err() {
            anyhow::bail!("Invalid URL.");
        }

        Ok(Self {
            id,
            name,
            icon_url,
            role,
        })
    }
}
