#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentFlowsAmountDetailsResourceTip {
    /// Portion of the amount that corresponds to a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}
