#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionPhoneNumberCollection {
    /// Indicates whether phone number collection is enabled for the session.
    pub enabled: bool,
}
