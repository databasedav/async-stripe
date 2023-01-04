#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ThreeDSecureUsage {
    /// Whether 3D Secure is supported on this card.
    pub supported: bool,
}
