#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LoginLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The URL for the login link.
    pub url: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostAccountsAccountLoginLinksParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn post_accounts_account_login_links(
    client: &crate::Client,
    account: String,
    params: PostAccountsAccountLoginLinksParams,
) -> crate::Response<crate::generated::LoginLink> {
    client.post_form(&format!("/accounts/{account}/login_links", account = account), params)
}
