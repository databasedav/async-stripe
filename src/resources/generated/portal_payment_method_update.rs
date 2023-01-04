#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
