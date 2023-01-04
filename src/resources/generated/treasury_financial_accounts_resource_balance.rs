/// Balance information for the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceBalance {
    /// Funds the user can spend right now.
    pub cash: i64,

    /// Funds not spendable yet, but will become available at a later time.
    pub inbound_pending: i64,

    /// Funds in the account, but not spendable because they are being held for pending outbound flows.
    pub outbound_pending: i64,
}
