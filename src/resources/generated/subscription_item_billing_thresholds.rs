#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionItemBillingThresholds {
    /// Usage threshold that triggers the subscription to create an invoice.
    pub usage_gte: Option<i64>,
}
