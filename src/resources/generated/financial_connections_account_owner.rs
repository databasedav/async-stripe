#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FinancialConnectionsAccountOwner {
    /// The email address of the owner.
    pub email: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// The full name of the owner.
    pub name: String,

    /// The ownership object that this owner belongs to.
    pub ownership: String,

    /// The raw phone number of the owner.
    pub phone: Option<String>,

    /// The raw physical address of the owner.
    pub raw_address: Option<String>,

    /// The timestamp of the refresh that updated this owner.
    pub refreshed_at: Option<crate::params::Timestamp>,
}
