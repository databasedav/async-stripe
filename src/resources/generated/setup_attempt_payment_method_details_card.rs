#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupAttemptPaymentMethodDetailsCard {
    /// Populated if this authorization used 3D Secure authentication.
    pub three_d_secure: Option<crate::generated::ThreeDSecureDetails>,
}
