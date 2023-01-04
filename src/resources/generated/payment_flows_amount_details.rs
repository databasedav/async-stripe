#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentFlowsAmountDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tip: Option<crate::generated::PaymentFlowsAmountDetailsResourceTip>,
}
