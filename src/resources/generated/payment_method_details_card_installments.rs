#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsCardInstallments {
    /// Installment plan selected for the payment.
    pub plan: Option<crate::generated::PaymentMethodDetailsCardInstallmentsPlan>,
}
