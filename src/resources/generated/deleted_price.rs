#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeletedPrice {
    /// Always true for a deleted object.
    pub deleted: bool,

    /// Unique identifier for the object.
    pub id: String,
}
