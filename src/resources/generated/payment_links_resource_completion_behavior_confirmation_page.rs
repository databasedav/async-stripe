#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceCompletionBehaviorConfirmationPage {
    /// The custom message that is displayed to the customer after the purchase is complete.
    pub custom_message: Option<String>,
}
