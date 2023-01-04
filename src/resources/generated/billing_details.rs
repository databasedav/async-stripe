#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct BillingDetails {
    /// Billing address.
    pub address: Option<crate::generated::Address>,

    /// Email address.
    pub email: Option<String>,

    /// Full name.
    pub name: Option<String>,

    /// Billing phone number (including extension).
    pub phone: Option<String>,
}
