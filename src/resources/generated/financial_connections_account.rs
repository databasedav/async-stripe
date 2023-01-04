/// A Financial Connections Account represents an account that exists outside of Stripe, to which you have been granted some degree of access.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FinancialConnectionsAccount {
    /// The account holder that this account belongs to.
    pub account_holder: Option<crate::generated::BankConnectionsResourceAccountholder>,

    /// The most recent information about the account's balance.
    pub balance: Option<crate::generated::BankConnectionsResourceBalance>,

    /// The state of the most recent attempt to refresh the account balance.
    pub balance_refresh: Option<crate::generated::BankConnectionsResourceBalanceRefresh>,

    /// The type of the account.
    ///
    /// Account category is further divided in `subcategory`.
    pub category: FinancialConnectionsAccountCategory,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// A human-readable name that has been assigned to this account, either by the account holder or by the institution.
    pub display_name: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// The name of the institution that holds this account.
    pub institution_name: String,

    /// The last 4 digits of the account number.
    ///
    /// If present, this will be 4 numeric characters.
    pub last4: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The most recent information about the account's owners.
    pub ownership: Option<Vec<crate::generated::FinancialConnectionsAccountOwnership>>,

    /// The state of the most recent attempt to refresh the account owners.
    pub ownership_refresh: Option<crate::generated::BankConnectionsResourceOwnershipRefresh>,

    /// The list of permissions granted by this account.
    pub permissions: Option<Vec<FinancialConnectionsAccountPermissions>>,

    /// The status of the link to the account.
    pub status: FinancialConnectionsAccountStatus,

    /// If `category` is `cash`, one of:
    ///
    ///  - `checking`
    ///  - `savings`
    ///  - `other`
    ///
    /// If `category` is `credit`, one of:
    ///
    ///  - `mortgage`
    ///  - `line_of_credit`
    ///  - `credit_card`
    ///  - `other`
    ///
    /// If `category` is `investment` or `other`, this will be `other`.
    pub subcategory: FinancialConnectionsAccountSubcategory,

    /// The [PaymentMethod type](https://stripe.com/docs/api/payment_methods/object#payment_method_object-type)(s) that can be created from this account.
    pub supported_payment_method_types: Vec<FinancialConnectionsAccountSupportedPaymentMethodTypes>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFinancialConnectionsAccountsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder: Option<GetFinancialConnectionsAccountsParamsAccountHolder>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFinancialConnectionsAccountsAccountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetFinancialConnectionsAccountsAccountOwnersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    pub ownership: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostFinancialConnectionsAccountsAccountRefreshParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The list of account features that you would like to refresh.
    pub features: Vec<PostFinancialConnectionsAccountsAccountRefreshParamsFeatures>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostFinancialConnectionsAccountsAccountDisconnectParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountCategory {
    Cash,
    Credit,
    Investment,
    Other,
}

impl FinancialConnectionsAccountCategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Credit => "credit",
            Self::Investment => "investment",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountCategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsAccountCategory {
    fn default() -> Self {
        Self::Cash
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountPermissions {
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl FinancialConnectionsAccountPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balances => "balances",
            Self::Ownership => "ownership",
            Self::PaymentMethod => "payment_method",
            Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsAccountPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountStatus {
    Active,
    Disconnected,
    Inactive,
}

impl FinancialConnectionsAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Disconnected => "disconnected",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsAccountStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSubcategory {
    Checking,
    CreditCard,
    LineOfCredit,
    Mortgage,
    Other,
    Savings,
}

impl FinancialConnectionsAccountSubcategory {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::CreditCard => "credit_card",
            Self::LineOfCredit => "line_of_credit",
            Self::Mortgage => "mortgage",
            Self::Other => "other",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountSubcategory {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountSubcategory {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsAccountSubcategory {
    fn default() -> Self {
        Self::Checking
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FinancialConnectionsAccountSupportedPaymentMethodTypes {
    Link,
    UsBankAccount,
}

impl FinancialConnectionsAccountSupportedPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Link => "link",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FinancialConnectionsAccountSupportedPaymentMethodTypes {
    fn default() -> Self {
        Self::Link
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFinancialConnectionsAccountsParamsAccountHolder {
    /// The ID of the Stripe account whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account: Option<String>,

    /// The ID of the Stripe customer whose accounts will be retrieved.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostFinancialConnectionsAccountsAccountRefreshParamsFeatures {
    Balance,
    Ownership,
}

impl PostFinancialConnectionsAccountsAccountRefreshParamsFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balance => "balance",
            Self::Ownership => "ownership",
        }
    }
}

impl AsRef<str> for PostFinancialConnectionsAccountsAccountRefreshParamsFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostFinancialConnectionsAccountsAccountRefreshParamsFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostFinancialConnectionsAccountsAccountRefreshParamsFeatures {
    fn default() -> Self {
        Self::Balance
    }
}
pub fn get_financial_connections_accounts(
    client: &crate::Client,
    params: GetFinancialConnectionsAccountsParams,
) -> crate::Response<crate::params::List<crate::generated::FinancialConnectionsAccount>> {
    client.get_query("/financial_connections/accounts", params)
}

pub fn get_financial_connections_accounts_account(
    client: &crate::Client,
    account: String,
    params: GetFinancialConnectionsAccountsAccountParams,
) -> crate::Response<crate::generated::FinancialConnectionsAccount> {
    client
        .get_query(&format!("/financial_connections/accounts/{account}", account = account), params)
}

pub fn get_financial_connections_accounts_account_owners(
    client: &crate::Client,
    account: String,
    params: GetFinancialConnectionsAccountsAccountOwnersParams,
) -> crate::Response<crate::params::List<crate::generated::FinancialConnectionsAccountOwner>> {
    client.get_query(
        &format!("/financial_connections/accounts/{account}/owners", account = account),
        params,
    )
}

pub fn post_financial_connections_accounts_account_refresh(
    client: &crate::Client,
    account: String,
    params: PostFinancialConnectionsAccountsAccountRefreshParams,
) -> crate::Response<crate::generated::FinancialConnectionsAccount> {
    client.post_form(
        &format!("/financial_connections/accounts/{account}/refresh", account = account),
        params,
    )
}

pub fn post_financial_connections_accounts_account_disconnect(
    client: &crate::Client,
    account: String,
    params: PostFinancialConnectionsAccountsAccountDisconnectParams,
) -> crate::Response<crate::generated::FinancialConnectionsAccount> {
    client.post_form(
        &format!("/financial_connections/accounts/{account}/disconnect", account = account),
        params,
    )
}
