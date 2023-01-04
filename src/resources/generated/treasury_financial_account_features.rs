/// Encodes whether a FinancialAccount has access to a particular Feature, with a `status` enum and associated `status_details`.
/// Stripe or the platform can control Features via the requested field.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountFeatures {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance:
        Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<crate::generated::TreasuryFinancialAccountsResourceFinancialAddressesFeatures>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers:
        Option<crate::generated::TreasuryFinancialAccountsResourceInboundTransfers>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows:
        Option<crate::generated::TreasuryFinancialAccountsResourceToggleSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments:
        Option<crate::generated::TreasuryFinancialAccountsResourceOutboundPayments>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers:
        Option<crate::generated::TreasuryFinancialAccountsResourceOutboundTransfers>,
}
