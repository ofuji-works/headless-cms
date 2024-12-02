use anyhow::{bail, Result};

pub struct ContentModel {
  pub name: String,
  pub api_identifier: String,
  pub description: Option<String>,
}

impl ContentModel {
    pub fn try_new(name: String, api_identifier: String, description: Option<String>) -> Result<Self> {

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

        Ok(Self { name, api_identifier, description })
    }
}

