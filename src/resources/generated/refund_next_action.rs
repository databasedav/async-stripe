#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RefundNextAction {
    /// Contains the refund details.
    pub display_details: Option<crate::generated::RefundNextActionDisplayDetails>,

    /// Type of the next action to perform.
    #[serde(rename = "type")]
    pub type_: String,
}
