#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceTransferData {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    pub amount: Option<i64>,

    /// ID of the Connected account receiving the transfer.
    pub destination: Vec<crate::generated::Account>,
}
