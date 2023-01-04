#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsAfterpayClearpay {
    /// Order identifier shown to the merchant in Afterpay’s online portal.
    pub reference: Option<String>,
}
