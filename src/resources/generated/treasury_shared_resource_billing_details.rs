#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasurySharedResourceBillingDetails {
    pub address: crate::generated::Address,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,
}
