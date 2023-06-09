/// InboundTransfers contains inbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceInboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<crate::generated::TreasuryFinancialAccountsResourceAchToggleSettings>,
}