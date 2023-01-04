#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct AccountCardIssuingSettings {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_acceptance: Option<crate::generated::CardIssuingAccountTermsOfService>,
}
