#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction {
    /// The [Payment Intent](https://stripe.com/docs/api/payment_intents/object) that funds were unapplied from.
    pub payment_intent: Vec<crate::generated::PaymentIntent>,
}
