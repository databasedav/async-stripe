#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionAutomaticTax {
    /// Whether Stripe automatically computes tax on this subscription.
    pub enabled: bool,
}
