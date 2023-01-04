#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoicesFromInvoice {
    /// The relation between this invoice and the cloned invoice.
    pub action: String,

    /// The invoice that was cloned.
    pub invoice: Vec<crate::generated::Invoice>,
}
