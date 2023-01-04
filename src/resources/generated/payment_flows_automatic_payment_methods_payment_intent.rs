#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentFlowsAutomaticPaymentMethodsPaymentIntent {
    /// Automatically calculates compatible payment methods.
    pub enabled: bool,
}
