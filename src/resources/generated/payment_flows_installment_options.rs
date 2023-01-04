#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentFlowsInstallmentOptions {
    pub enabled: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<crate::generated::PaymentMethodDetailsCardInstallmentsPlan>,
}
