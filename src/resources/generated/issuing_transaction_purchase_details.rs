#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingTransactionPurchaseDetails {
    /// Information about the flight that was purchased with this transaction.
    pub flight: Option<crate::generated::IssuingTransactionFlightData>,

    /// Information about fuel that was purchased with this transaction.
    pub fuel: Option<crate::generated::IssuingTransactionFuelData>,

    /// Information about lodging that was purchased with this transaction.
    pub lodging: Option<crate::generated::IssuingTransactionLodgingData>,

    /// The line items in the purchase.
    pub receipt: Option<Vec<crate::generated::IssuingTransactionReceiptData>>,

    /// A merchant-specific order number.
    pub reference: Option<String>,
}
