#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsOxxo {
    /// OXXO reference number.
    pub number: Option<String>,
}
