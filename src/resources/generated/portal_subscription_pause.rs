#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalSubscriptionPause {
    /// Whether the feature is enabled.
    pub enabled: bool,
}
