/// OutboundTransfers contains outbound transfers features for a FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundTransfers {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<crate::generated::TreasuryFinancialAccountsResourceAchToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,
}
