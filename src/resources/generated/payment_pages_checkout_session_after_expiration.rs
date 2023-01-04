#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionAfterExpiration {
    /// When set, configuration used to recover the Checkout Session on expiry.
    pub recovery: Option<crate::generated::PaymentPagesCheckoutSessionAfterExpirationRecovery>,
}
