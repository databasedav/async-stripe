#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were applied to.
    pub payment_intent: Vec<crate::generated::PaymentIntent>,
}
