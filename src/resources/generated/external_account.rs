/// The resource representing a Stripe Polymorphic.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ExternalAccount {
    BankAccount(crate::generated::BankAccount),
    Card(crate::generated::Card),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetAccountsAccountExternalAccountsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetAccountsAccountExternalAccountsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostAccountsAccountExternalAccountsParams {
    /// When set to true, or if this is the first external account added in this currency, this account becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Please refer to full [documentation](https://stripe.com/docs/api) instead.
    pub external_account: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostAccountsAccountExternalAccountsIdParams {
    /// The name of the person or business that owns the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The type of entity that holds the account.
    ///
    /// This can be either `individual` or `company`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostAccountsAccountExternalAccountsIdParamsAccountHolderType>,

    /// The bank account type.
    ///
    /// This can only be `checking` or `savings` in most countries.
    /// In Japan, this can only be `futsu` or `toza`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PostAccountsAccountExternalAccountsIdParamsAccountType>,

    /// City/District/Suburb/Town/Village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_city: Option<String>,

    /// Billing address country, if provided when creating card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_country: Option<String>,

    /// Address line 1 (Street address/PO Box/Company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line1: Option<String>,

    /// Address line 2 (Apartment/Suite/Unit/Building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_line2: Option<String>,

    /// State/County/Province/Region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_state: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address_zip: Option<String>,

    /// When set to true, this becomes the default external account for its currency.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_for_currency: Option<bool>,

    /// Two digit number representing the card’s expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<String>,

    /// Four digit number representing the card’s expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Cardholder name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    Company,
    Individual,
}

impl PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountsAccountExternalAccountsIdParamsAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostAccountsAccountExternalAccountsIdParamsAccountType {
    Checking,
    Futsu,
    Savings,
    Toza,
}

impl PostAccountsAccountExternalAccountsIdParamsAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Futsu => "futsu",
            Self::Savings => "savings",
            Self::Toza => "toza",
        }
    }
}

impl AsRef<str> for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostAccountsAccountExternalAccountsIdParamsAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
pub fn get_accounts_account_external_accounts(
    client: &crate::Client,
    account: String,
    params: GetAccountsAccountExternalAccountsParams,
) -> crate::Response<crate::params::List<crate::generated::ExternalAccount>> {
    client.get_query(&format!("/accounts/{account}/external_accounts", account = account), params)
}

pub fn get_accounts_account_external_accounts_id(
    client: &crate::Client,
    account: String,
    id: String,
    params: GetAccountsAccountExternalAccountsIdParams,
) -> crate::Response<crate::generated::ExternalAccount> {
    client.get_query(
        &format!("/accounts/{account}/external_accounts/{id}", account = account, id = id),
        params,
    )
}

pub fn post_accounts_account_external_accounts(
    client: &crate::Client,
    account: String,
    params: PostAccountsAccountExternalAccountsParams,
) -> crate::Response<crate::generated::ExternalAccount> {
    client.post_form(&format!("/accounts/{account}/external_accounts", account = account), params)
}

pub fn post_accounts_account_external_accounts_id(
    client: &crate::Client,
    account: String,
    id: String,
    params: PostAccountsAccountExternalAccountsIdParams,
) -> crate::Response<crate::generated::ExternalAccount> {
    client.post_form(
        &format!("/accounts/{account}/external_accounts/{id}", account = account, id = id),
        params,
    )
}

pub fn delete_accounts_account_external_accounts_id(
    client: &crate::Client,
    account: String,
    id: String,
) -> crate::Response<crate::generated::DeletedExternalAccount> {
    client.delete(&format!(
        "/accounts/{account}/external_accounts/{id}",
        account = account,
        id = id
    ))
}
