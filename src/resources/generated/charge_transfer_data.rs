#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ChargeTransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: Vec<crate::generated::Account>,
}
