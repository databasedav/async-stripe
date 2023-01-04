#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalSubscriptionUpdateProduct {
    /// The list of price IDs which, when subscribed to, a subscription can be updated.
    pub prices: Vec<String>,

    /// The product ID.
    pub product: String,
}
