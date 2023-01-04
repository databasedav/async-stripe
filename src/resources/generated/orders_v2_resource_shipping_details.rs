#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceShippingDetails {
    /// Recipient shipping address.
    ///
    /// Required if the order includes products that are shippable.
    pub address: Option<crate::generated::Address>,

    /// Recipient name.
    pub name: Option<String>,

    /// Recipient phone (including extension).
    pub phone: Option<String>,
}
