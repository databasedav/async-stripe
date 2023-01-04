#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MandatePaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::MandateAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::MandateAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::MandateBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::MandateBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::CardMandatePaymentMethodDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::MandateLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::MandateSepaDebit>,

    /// The type of the payment method associated with this mandate.
    ///
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains mandate information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::MandateUsBankAccount>,
}
