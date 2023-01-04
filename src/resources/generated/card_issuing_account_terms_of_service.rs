#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CardIssuingAccountTermsOfService {
    /// The Unix timestamp marking when the account representative accepted the service agreement.
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the account representative accepted the service agreement.
    pub ip: Option<String>,

    /// The user agent of the browser from which the account representative accepted the service agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}
