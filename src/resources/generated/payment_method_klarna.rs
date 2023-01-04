#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodKlarna {
    /// The customer's date of birth, if provided.
    pub dob: Option<crate::generated::PaymentFlowsPrivatePaymentMethodsKlarnaDob>,
}
