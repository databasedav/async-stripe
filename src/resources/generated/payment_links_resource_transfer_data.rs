#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceTransferData {
    /// The amount in %s that will be transferred to the destination account.
    ///
    /// By default, the entire amount is transferred to the destination.
    pub amount: Option<i64>,

    /// The connected account receiving the transfer.
    pub destination: Vec<crate::generated::Account>,
}
