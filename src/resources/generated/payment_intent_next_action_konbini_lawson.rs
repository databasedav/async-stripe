#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionKonbiniLawson {
    /// The confirmation number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub confirmation_number: Option<String>,

    /// The payment code.
    pub payment_code: String,
}
