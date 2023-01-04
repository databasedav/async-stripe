#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoicesLineItemsProrationDetails {
    /// For a credit proration `line_item`, the original debit line_items to which the credit proration applies.
    pub credited_items: Option<crate::generated::InvoicesLineItemsCreditedItems>,
}
