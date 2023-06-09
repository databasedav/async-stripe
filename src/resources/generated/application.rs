use std::str::FromStr;

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the application.
    pub name: Option<String>,
}

impl crate::params::Object for Application {
    type Id = crate::ids::ApplicationId;
    fn id(&self) -> Self::Id {
        crate::ids::ApplicationId::from_str(&self.id).unwrap()
    }
    fn object(&self) -> &'static str {
        "application"
    }
}
