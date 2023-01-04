#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}
