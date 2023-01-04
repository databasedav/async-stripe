/// A Financial Connections Session is the secure way to programmatically launch the client-side Stripe.js modal that lets your users link their accounts.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FinancialConnectionsSession {
    /// The account holder for whom accounts are collected in this session.
    pub account_holder: Option<crate::generated::BankConnectionsResourceAccountholder>,

    /// The accounts that were collected as part of this Session.
    pub accounts: crate::params::List<crate::generated::FinancialConnectionsAccount>,

    /// A value that will be passed to the client to launch the authentication flow.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<crate::generated::BankConnectionsResourceLinkAccountSessionFilters>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Permissions requested for accounts collected during this session.
    pub permissions: Vec<FinancialConnectionsSessionPermissions>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostFinancialConnectionsSessionsParams {
    /// The account holder to link accounts for.
    pub account_holder: PostFinancialConnectionsSessionsParamsAccountHolder,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Filters to restrict the kinds of accounts to collect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filters: Option<PostFinancialConnectionsSessionsParamsFilters>,

    /// List of data features that you would like to request access to.
    ///
    /// Possible values are `balances`, `transactions`, `ownership`, and `payment_method`.
    pub permissions: Vec<PostFinancialConnectionsSessionsParamsPermissions>,

    /// For webview integrations only.
    ///
    /// Upon completing OAuth login in the native browser, the user will be redirected to this URL to return to your app.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFinancialConnectionsSessionsSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsSessionPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsSessionPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsSessionPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsSessionPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsSessionPermissions {
    fn default() -> Self {
        Self::Balances
    }
}
/// The account holder to link accounts for.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostFinancialConnectionsSessionsParamsAccountHolder {
    /// The ID of the Stripe account whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// The ID of the Stripe customer whose accounts will be retrieved.
    ///
    /// Should only be present if `type` is `customer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Type of account holder to collect accounts for.
    #[serde(rename = "type")]
    pub type_: PostFinancialConnectionsSessionsParamsAccountHolderType,
}

/// Filters to restrict the kinds of accounts to collect.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostFinancialConnectionsSessionsParamsFilters {
    /// List of countries from which to collect accounts.
    pub countries: Vec<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostFinancialConnectionsSessionsParamsPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl PostFinancialConnectionsSessionsParamsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for PostFinancialConnectionsSessionsParamsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostFinancialConnectionsSessionsParamsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostFinancialConnectionsSessionsParamsPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostFinancialConnectionsSessionsParamsAccountHolderType {
    Account,
    Customer,
}

impl PostFinancialConnectionsSessionsParamsAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Account => "account",
            Self::Customer => "customer",
        }
    }
}

impl AsRef<str> for PostFinancialConnectionsSessionsParamsAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostFinancialConnectionsSessionsParamsAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostFinancialConnectionsSessionsParamsAccountHolderType {
    fn default() -> Self {
        Self::Account
    }
}
pub fn post_financial_connections_sessions(
    client: &crate::Client,
    params: PostFinancialConnectionsSessionsParams,
) -> crate::Response<crate::generated::FinancialConnectionsSession> {
    client.post_form("/financial_connections/sessions", params)
}

pub fn get_financial_connections_sessions_session(
    client: &crate::Client,
    session: String,
    params: GetFinancialConnectionsSessionsSessionParams,
) -> crate::Response<crate::generated::FinancialConnectionsSession> {
    client
        .get_query(&format!("/financial_connections/sessions/{session}", session = session), params)
}
