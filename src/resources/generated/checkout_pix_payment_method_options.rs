#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CheckoutPixPaymentMethodOptions {
    /// The number of seconds after which Pix payment will expire.
    pub expires_after_seconds: Option<i64>,
}
