#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceTaxIdCollection {
    /// Indicates whether tax ID collection is enabled for the session.
    pub enabled: bool,
}
