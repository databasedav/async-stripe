#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceAutomaticTax {
    /// If `true`, tax will be calculated automatically using the customer's location.
    pub enabled: bool,
}
