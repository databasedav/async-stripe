#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    pub amount_percent: Option<f64>,

    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: Vec<crate::generated::Account>,
}
