#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourcePhoneNumberCollection {
    /// If `true`, a phone number will be collected during checkout.
    pub enabled: bool,
}
