#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeletedApplication {
    /// Always true for a deleted object.
    pub deleted: bool,

    /// Unique identifier for the object.
    pub id: String,

    /// The name of the application.
    pub name: Option<String>,
}
