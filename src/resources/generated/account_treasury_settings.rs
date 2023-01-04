#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AccountTreasurySettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<crate::generated::AccountTermsOfService>,
}
