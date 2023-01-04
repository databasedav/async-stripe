/// Settings related to Financial Addresses features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddressesFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,
}
