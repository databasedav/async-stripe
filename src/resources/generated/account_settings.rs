#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct AccountSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit_payments: Option<crate::generated::AccountBacsDebitPaymentsSettings>,

    pub branding: crate::generated::AccountBrandingSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<crate::generated::AccountCardIssuingSettings>,

    pub card_payments: crate::generated::AccountCardPaymentsSettings,

    pub dashboard: crate::generated::AccountDashboardSettings,

    pub payments: crate::generated::AccountPaymentsSettings,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payouts: Option<crate::generated::AccountPayoutSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit_payments: Option<crate::generated::AccountSepaDebitPaymentsSettings>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<crate::generated::AccountTreasurySettings>,
}
