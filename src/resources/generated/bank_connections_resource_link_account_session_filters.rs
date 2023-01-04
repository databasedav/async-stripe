#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct BankConnectionsResourceLinkAccountSessionFilters {
    /// List of countries from which to filter accounts.
    pub countries: Option<Vec<String>>,
}
