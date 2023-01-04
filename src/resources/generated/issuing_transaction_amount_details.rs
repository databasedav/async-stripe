#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingTransactionAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
