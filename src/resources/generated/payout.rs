/// A `Payout` object is created when you receive funds from Stripe, or when you
/// initiate a payout to either a bank account or debit card of a [connected
/// Stripe account](/docs/connect/bank-debit-card-payouts).
///
/// You can retrieve individual payouts, as well as list all payouts.
/// Payouts are made on [varying schedules](/docs/connect/manage-payout-schedule), depending on your country and industry.  Related guide: [Receiving Payouts](https://stripe.com/docs/payouts).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Payout {
    /// Amount (in %s) to be transferred to your bank account or debit card.
    pub amount: i64,

    /// Date the payout is expected to arrive in the bank.
    ///
    /// This factors in delays like weekends or bank holidays.
    pub arrival_date: crate::params::Timestamp,

    /// Returns `true` if the payout was created by an [automated payout schedule](https://stripe.com/docs/payouts#payout-schedule), and `false` if it was [requested manually](https://stripe.com/docs/payouts#manual-payouts).
    pub automatic: bool,

    /// ID of the balance transaction that describes the impact of this payout on your account balance.
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// ID of the bank account or card the payout was sent to.
    pub destination: Option<Vec<crate::generated::ExternalAccount>>,

    /// If the payout failed or was canceled, this will be the ID of the balance transaction that reversed the initial balance transaction, and puts the funds from the failed payout back in your balance.
    pub failure_balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Error code explaining reason for payout failure if available.
    ///
    /// See [Types of payout failures](https://stripe.com/docs/api#payout_failures) for a list of failure codes.
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for payout failure if available.
    pub failure_message: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces](https://stripe.com/blog/instant-payouts-for-marketplaces) for more information.).
    pub method: String,

    /// If the payout reverses another, this is the ID of the original payout.
    pub original_payout: Option<Vec<crate::generated::Payout>>,

    /// If the payout was reversed, this is the ID of the payout that reverses this payout.
    pub reversed_by: Option<Vec<crate::generated::Payout>>,

    /// The source balance this payout came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    pub source_type: String,

    /// Extra information about a payout to be displayed on the user's bank statement.
    pub statement_descriptor: Option<String>,

    /// Current status of the payout: `paid`, `pending`, `in_transit`, `canceled` or `failed`.
    ///
    /// A payout is `pending` until it is submitted to the bank, when it becomes `in_transit`.
    /// The status then changes to `paid` if the transaction goes through, or to `failed` or `canceled` (within 5 business days).
    /// Some failed payouts may initially show as `paid` but then change to `failed`.
    pub status: String,

    /// Can be `bank_account` or `card`.
    #[serde(rename = "type")]
    pub type_: PayoutType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetPayoutsPayoutParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetPayoutsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arrival_date: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPayoutsParams {
    /// A positive integer in cents representing how much to payout.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The ID of a bank account or a card to send the payout to.
    ///
    /// If no destination is supplied, the default external account for the specified currency will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

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

    /// The method used to send this payout, which can be `standard` or `instant`.
    ///
    /// `instant` is only supported for payouts to debit cards.
    /// (See [Instant payouts for marketplaces for more information](https://stripe.com/blog/instant-payouts-for-marketplaces).).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub method: Option<PostPayoutsParamsMethod>,

    /// The balance type of your Stripe balance to draw this payout from.
    ///
    /// Balances for different payment sources are kept separately.
    /// You can find the amounts with the balances API.
    /// One of `bank_account`, `card`, or `fpx`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PostPayoutsParamsSourceType>,

    /// A string to be displayed on the recipient's bank or card statement.
    ///
    /// This may be at most 22 characters.
    /// Attempting to use a `statement_descriptor` longer than 22 characters will return an error.
    /// Note: Most banks will truncate this information and/or display it inconsistently.
    /// Some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPayoutsPayoutParams {
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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPayoutsPayoutCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPayoutsPayoutReverseParams {
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
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PayoutType {
    BankAccount,
    Card,
}

impl PayoutType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::Card => "card",
        }
    }
}

impl AsRef<str> for PayoutType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PayoutType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PayoutType {
    fn default() -> Self {
        Self::BankAccount
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPayoutsParamsMethod {
    Instant,
    Standard,
}

impl PostPayoutsParamsMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Instant => "instant",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for PostPayoutsParamsMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPayoutsParamsMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPayoutsParamsMethod {
    fn default() -> Self {
        Self::Instant
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPayoutsParamsSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl PostPayoutsParamsSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::Card => "card",
            Self::Fpx => "fpx",
        }
    }
}

impl AsRef<str> for PostPayoutsParamsSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPayoutsParamsSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPayoutsParamsSourceType {
    fn default() -> Self {
        Self::BankAccount
    }
}
pub fn get_payouts_payout(
    client: &crate::Client,
    payout: String,
    params: GetPayoutsPayoutParams,
) -> crate::Response<crate::generated::Payout> {
    client.get_query(&format!("/payouts/{payout}", payout = payout), params)
}

pub fn get_payouts(
    client: &crate::Client,
    params: GetPayoutsParams,
) -> crate::Response<crate::params::List<crate::generated::Payout>> {
    client.get_query("/payouts", params)
}

pub fn post_payouts(
    client: &crate::Client,
    params: PostPayoutsParams,
) -> crate::Response<crate::generated::Payout> {
    client.post_form("/payouts", params)
}

pub fn post_payouts_payout(
    client: &crate::Client,
    payout: String,
    params: PostPayoutsPayoutParams,
) -> crate::Response<crate::generated::Payout> {
    client.post_form(&format!("/payouts/{payout}", payout = payout), params)
}

pub fn post_payouts_payout_cancel(
    client: &crate::Client,
    payout: String,
    params: PostPayoutsPayoutCancelParams,
) -> crate::Response<crate::generated::Payout> {
    client.post_form(&format!("/payouts/{payout}/cancel", payout = payout), params)
}

pub fn post_payouts_payout_reverse(
    client: &crate::Client,
    payout: String,
    params: PostPayoutsPayoutReverseParams,
) -> crate::Response<crate::generated::Payout> {
    client.post_form(&format!("/payouts/{payout}/reverse", payout = payout), params)
}
