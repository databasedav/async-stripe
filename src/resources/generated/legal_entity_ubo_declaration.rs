#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct LegalEntityUboDeclaration {
    /// The Unix timestamp marking when the beneficial owner attestation was made.
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the beneficial owner attestation was made.
    pub ip: Option<String>,

    /// The user-agent string from the browser where the beneficial owner attestation was made.
    pub user_agent: Option<String>,
}
