/// This is an object representing a capability for a Stripe account.
///
/// Related guide: [Account capabilities](https://stripe.com/docs/connect/account-capabilities).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Capability {
    /// The account for which the capability enables functionality.
    pub account: Vec<crate::generated::Account>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub future_requirements: Option<crate::generated::AccountCapabilityFutureRequirements>,

    /// The identifier for the capability.
    pub id: String,

    /// Whether the capability has been requested.
    pub requested: bool,

    /// Time at which the capability was requested.
    ///
    /// Measured in seconds since the Unix epoch.
    pub requested_at: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub requirements: Option<crate::generated::AccountCapabilityRequirements>,

    /// The status of the capability.
    ///
    /// Can be `active`, `inactive`, `pending`, or `unrequested`.
    pub status: CapabilityStatus,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetAccountsAccountCapabilitiesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetAccountsAccountCapabilitiesCapabilityParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostAccountsAccountCapabilitiesCapabilityParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Passing true requests the capability for the account, if it is not already requested.
    ///
    /// A requested capability may not immediately become active.
    /// Any requirements to activate the capability are returned in the `requirements` arrays.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CapabilityStatus {
    Active,
    Disabled,
    Inactive,
    Pending,
    Unrequested,
}

impl CapabilityStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disabled => "disabled",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
            Self::Unrequested => "unrequested",
        }
    }
}

impl AsRef<str> for CapabilityStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CapabilityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CapabilityStatus {
    fn default() -> Self {
        Self::Active
    }
}
pub fn get_accounts_account_capabilities(
    client: &crate::Client,
    account: String,
    params: GetAccountsAccountCapabilitiesParams,
) -> crate::Response<crate::params::List<crate::generated::Capability>> {
    client.get_query(&format!("/accounts/{account}/capabilities", account = account), params)
}

pub fn get_accounts_account_capabilities_capability(
    client: &crate::Client,
    account: String,
    capability: String,
    params: GetAccountsAccountCapabilitiesCapabilityParams,
) -> crate::Response<crate::generated::Capability> {
    client.get_query(
        &format!(
            "/accounts/{account}/capabilities/{capability}",
            account = account,
            capability = capability
        ),
        params,
    )
}

pub fn post_accounts_account_capabilities_capability(
    client: &crate::Client,
    account: String,
    capability: String,
    params: PostAccountsAccountCapabilitiesCapabilityParams,
) -> crate::Response<crate::generated::Capability> {
    client.post_form(
        &format!(
            "/accounts/{account}/capabilities/{capability}",
            account = account,
            capability = capability
        ),
        params,
    )
}
