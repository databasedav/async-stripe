#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction {
    /// The [Refund](https://stripe.com/docs/api/refunds/object) that moved these funds into the customer's cash balance.
    pub refund: Vec<crate::generated::Refund>,
}
