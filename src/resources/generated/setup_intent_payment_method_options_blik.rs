#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsBlik {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<crate::generated::SetupIntentPaymentMethodOptionsMandateOptionsBlik>,
}
