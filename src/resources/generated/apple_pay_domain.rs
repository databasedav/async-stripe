#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ApplePayDomain {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    pub domain_name: String,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetApplePayDomainsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub domain_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostApplePayDomainsParams {
    pub domain_name: String,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetApplePayDomainsDomainParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_apple_pay_domains(
    client: &crate::Client,
    params: GetApplePayDomainsParams,
) -> crate::Response<crate::params::List<crate::generated::ApplePayDomain>> {
    client.get_query("/apple_pay/domains", params)
}

pub fn post_apple_pay_domains(
    client: &crate::Client,
    params: PostApplePayDomainsParams,
) -> crate::Response<crate::generated::ApplePayDomain> {
    client.post_form("/apple_pay/domains", params)
}

pub fn get_apple_pay_domains_domain(
    client: &crate::Client,
    domain: String,
    params: GetApplePayDomainsDomainParams,
) -> crate::Response<crate::generated::ApplePayDomain> {
    client.get_query(&format!("/apple_pay/domains/{domain}", domain = domain), params)
}

pub fn delete_apple_pay_domains_domain(
    client: &crate::Client,
    domain: String,
) -> crate::Response<crate::generated::DeletedApplePayDomain> {
    client.delete(&format!("/apple_pay/domains/{domain}", domain = domain))
}
