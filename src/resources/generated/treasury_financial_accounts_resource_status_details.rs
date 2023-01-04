#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceStatusDetails {
    /// Details related to the closure of this FinancialAccount.
    pub closed: Option<crate::generated::TreasuryFinancialAccountsResourceClosedStatusDetails>,
}
