#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoiceTransferData {
    /// The amount in %s that will be transferred to the destination account when the invoice is paid.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,

    /// The account where funds from the payment will be transferred to upon payment success.
    pub destination: Vec<crate::generated::Account>,
}