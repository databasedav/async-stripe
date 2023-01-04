#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodOptionsCardInstallments {
    /// Installment plans that may be selected for this PaymentIntent.
    pub available_plans: Option<Vec<crate::generated::PaymentMethodDetailsCardInstallmentsPlan>>,

    /// Whether Installments are enabled for this PaymentIntent.
    pub enabled: bool,

    /// Installment plan selected for this PaymentIntent.
    pub plan: Option<crate::generated::PaymentMethodDetailsCardInstallmentsPlan>,
}
