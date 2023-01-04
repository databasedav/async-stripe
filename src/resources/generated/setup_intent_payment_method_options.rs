#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::SetupIntentPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::SetupIntentPaymentMethodOptionsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::SetupIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::SetupIntentPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::SetupIntentPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::SetupIntentPaymentMethodOptionsUsBankAccount>,
}
