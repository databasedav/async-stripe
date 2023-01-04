#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentCardProcessing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_notification:
        Option<crate::generated::PaymentIntentProcessingCustomerNotification>,
}
