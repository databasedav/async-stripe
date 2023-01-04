/// Settings related to Outbound Payments features on a Financial Account.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceOutboundPayments {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<crate::generated::TreasuryFinancialAccountsResourceAchToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,
}
