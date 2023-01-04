#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoiceThresholdReason {
    /// The total invoice amount threshold boundary if it triggered the threshold invoice.
    pub amount_gte: Option<i64>,

    /// Indicates which line items triggered a threshold invoice.
    pub item_reasons: Vec<crate::generated::InvoiceItemThresholdReason>,
}
