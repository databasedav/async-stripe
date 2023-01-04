#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoicesLineItemsCreditedItems {
    /// Invoice containing the credited invoice line items.
    pub invoice: String,

    /// Credited invoice line items.
    pub invoice_line_items: Vec<String>,
}
