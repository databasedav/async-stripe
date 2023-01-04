#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingAuthorizationAmountDetails {
    /// The fee charged by the ATM for the cash withdrawal.
    pub atm_fee: Option<i64>,
}
