#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeletedWebhookEndpoint {
    /// Always true for a deleted object.
    pub deleted: bool,

    /// Unique identifier for the object.
    pub id: String,
}
