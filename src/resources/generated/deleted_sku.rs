#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeletedSku {
    /// Always true for a deleted object.
    pub deleted: bool,

    /// Unique identifier for the object.
    pub id: String,
}
