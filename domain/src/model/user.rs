pub trait Role {
    fn permissions() -> std::collections::HashMap<&'static str, &'static str>;
}

pub struct Admin;
pub struct Member;

type Authorities = std::collections::HashMap<&'static str, &'static str>;

impl Role for Admin {
    fn permissions() -> Authorities {
        let mut permissions = std::collections::HashMap::new();
        permissions.insert("contents", "get,find,create,update,delete");

        permissions
    }
}
impl Role for Member {
    fn permissions() -> Authorities {
        let mut permissions = std::collections::HashMap::new();
        permissions.insert("contents", "get");

        permissions
    }
}

#[derive(Debug, serde::Serialize, serde::Deserialize, utoipa::ToSchema, Clone)]
pub struct User<T: Role> {
    pub id: String,
    pub name: String,
    pub icon_url: String,
    #[serde(skip_deserializing)]
    pub _role: std::marker::PhantomData<T>,
}

impl<T: Role> User<T> {
    pub fn try_new(id: String, name: String, icon_url: String) -> anyhow::Result<Self> {
        if name.len() < 1 {
            anyhow::bail!("Name must be at least 1 character long");
        }

        if name.len() > 50 {
            anyhow::bail!("Name exceeds maximum length(50 characters)");
        }

        if url::Url::parse(&icon_url).is_err() {
            anyhow::bail!("Icon URL is invalid URL.");
        }

        Ok(Self {
            id,
            name,
            icon_url,
            _role: std::marker::PhantomData,
        })
    }

    pub fn permissions(&self) -> Authorities {
        T::permissions()
    }
}
