#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsLink {
    /// Token used for persistent Link logins.
    pub persistent_token: Option<String>,
}
