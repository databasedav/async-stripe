/// ReceivedDebits represent funds pulled from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// These are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedDebit {
    /// Amount (in cents) transferred.
pub amount: i64,

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
pub description: String,

    /// Reason for the failure.
    ///
    /// A ReceivedDebit might fail because the FinancialAccount doesn't have sufficient funds, is closed, or is frozen.
pub failure_code: Option<TreasuryReceivedDebitFailureCode>,

    /// The FinancialAccount that funds were pulled from.
pub financial_account: Option<String>,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
pub id: String,

#[serde(skip_serializing_if = "Option::is_none")]
pub initiating_payment_method_details: Option<crate::generated::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails>,

pub linked_flows: crate::generated::TreasuryReceivedDebitsResourceLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,

    /// The network used for the ReceivedDebit.
pub network: TreasuryReceivedDebitNetwork,

    /// Details describing when a ReceivedDebit might be reversed.
pub reversal_details: Option<crate::generated::TreasuryReceivedDebitsResourceReversalDetails>,

    /// Status of the ReceivedDebit.
    ///
    /// ReceivedDebits are created with a status of either `succeeded` (approved) or `failed` (declined).
    /// The failure reason can be found under the `failure_code`.
pub status: TreasuryReceivedDebitStatus,

    /// The Transaction associated with this object.
pub transaction: Option<Vec<crate::generated::TreasuryTransaction>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryReceivedDebitsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTreasuryReceivedDebitsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryReceivedDebitsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedDebitsParams {
    /// Amount (in cents) to be transferred.
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

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The FinancialAccount to pull funds from.
    pub financial_account: String,

    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetails>,

    /// The rails used for the object.
    pub network: PostTestHelpersTreasuryReceivedDebitsParamsNetwork,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitFailureCode {
    AccountClosed,
    AccountFrozen,
    InsufficientFunds,
    Other,
}

impl TreasuryReceivedDebitFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::InsufficientFunds => "insufficient_funds",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedDebitFailureCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitNetwork {
    Ach,
    Card,
    Stripe,
}

impl TreasuryReceivedDebitNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
            Self::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedDebitNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedDebitStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryReceivedDebitStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedDebitStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryReceivedDebitsParamsStatus {
    Failed,
    Succeeded,
}

impl GetTreasuryReceivedDebitsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for GetTreasuryReceivedDebitsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryReceivedDebitsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryReceivedDebitsParamsStatus {
    fn default() -> Self {
        Self::Failed
    }
}
/// Initiating payment method details for the object.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetails {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType,

    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<
        PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsUsBankAccount,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryReceivedDebitsParamsNetwork {
    Ach,
}

impl PostTestHelpersTreasuryReceivedDebitsParamsNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
        }
    }
}

impl AsRef<str> for PostTestHelpersTreasuryReceivedDebitsParamsNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTestHelpersTreasuryReceivedDebitsParamsNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryReceivedDebitsParamsNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedDebitsParamsInitiatingPaymentMethodDetailsUsBankAccount {
    /// The bank account holder's name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_name: Option<String>,

    /// The bank account number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// The bank account's routing number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

pub fn get_treasury_received_debits(
    client: &crate::Client,
    params: GetTreasuryReceivedDebitsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryReceivedDebit>> {
    client.get_query("/treasury/received_debits", params)
}

pub fn get_treasury_received_debits_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryReceivedDebitsIdParams,
) -> crate::Response<crate::generated::TreasuryReceivedDebit> {
    client.get_query(&format!("/treasury/received_debits/{id}", id = id), params)
}

pub fn post_test_helpers_treasury_received_debits(
    client: &crate::Client,
    params: PostTestHelpersTreasuryReceivedDebitsParams,
) -> crate::Response<crate::generated::TreasuryReceivedDebit> {
    client.post_form("/test_helpers/treasury/received_debits", params)
}
