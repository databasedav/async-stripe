#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Application {
    /// Unique identifier for the object.
    pub id: String,

    /// The name of the application.
    pub name: Option<String>,
}
