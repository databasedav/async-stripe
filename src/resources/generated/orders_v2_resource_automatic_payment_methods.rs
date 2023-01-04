#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceAutomaticPaymentMethods {
    /// Whether this Order has been opted into managing payment method types via the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub enabled: bool,
}
