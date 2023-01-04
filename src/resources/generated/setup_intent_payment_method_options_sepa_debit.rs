#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<crate::generated::SetupIntentPaymentMethodOptionsMandateOptionsSepaDebit>,
}
