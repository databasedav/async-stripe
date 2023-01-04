#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AccountSepaDebitPaymentsSettings {
    /// SEPA creditor identifier that identifies the company making the payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub creditor_id: Option<String>,
}
