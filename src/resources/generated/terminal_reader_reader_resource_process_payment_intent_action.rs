/// Represents a reader action to process a payment intent.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalReaderReaderResourceProcessPaymentIntentAction {
    /// Most recent PaymentIntent processed by the reader.
    pub payment_intent: Vec<crate::generated::PaymentIntent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_config: Option<crate::generated::TerminalReaderReaderResourceProcessConfig>,
}
