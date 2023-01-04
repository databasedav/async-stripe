#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceBillingDetails {
    /// Billing address for the order.
    pub address: Option<crate::generated::Address>,

    /// Email address for the order.
    pub email: Option<String>,

    /// Full name for the order.
    pub name: Option<String>,

    /// Billing phone number for the order (including extension).
    pub phone: Option<String>,
}
