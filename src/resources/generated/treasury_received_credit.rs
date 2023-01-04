/// ReceivedCredits represent funds sent to a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) (for example, via ACH or wire).
///
/// These money movements are not initiated from the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedCredit {
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
    /// A ReceivedCredit might fail because the receiving FinancialAccount is closed or frozen.
pub failure_code: Option<TreasuryReceivedCreditFailureCode>,

    /// The FinancialAccount that received the funds.
pub financial_account: Option<String>,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
pub id: String,

pub initiating_payment_method_details: crate::generated::TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails,

pub linked_flows: crate::generated::TreasuryReceivedCreditsResourceLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,

    /// The rails used to send the funds.
pub network: TreasuryReceivedCreditNetwork,

    /// Details describing when a ReceivedCredit may be reversed.
pub reversal_details: Option<crate::generated::TreasuryReceivedCreditsResourceReversalDetails>,

    /// Status of the ReceivedCredit.
    ///
    /// ReceivedCredits are created either `succeeded` (approved) or `failed` (declined).
    /// If a ReceivedCredit is declined, the failure reason can be found in the `failure_code` field.
pub status: TreasuryReceivedCreditStatus,

    /// The Transaction associated with this object.
pub transaction: Option<Vec<crate::generated::TreasuryTransaction>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryReceivedCreditsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub linked_flows: Option<GetTreasuryReceivedCreditsParamsLinkedFlows>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTreasuryReceivedCreditsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryReceivedCreditsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedCreditsParams {
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

    /// The FinancialAccount to send funds to.
    pub financial_account: String,

    /// Initiating payment method details for the object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub initiating_payment_method_details:
        Option<PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetails>,

    /// The rails used for the object.
    pub network: PostTestHelpersTreasuryReceivedCreditsParamsNetwork,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditFailureCode {
    AccountClosed,
    AccountFrozen,
    Other,
}

impl TreasuryReceivedCreditFailureCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditFailureCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditFailureCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedCreditFailureCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditNetwork {
    Ach,
    Card,
    Stripe,
    UsDomesticWire,
}

impl TreasuryReceivedCreditNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
            Self::Stripe => "stripe",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedCreditNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditStatus {
    Failed,
    Succeeded,
}

impl TreasuryReceivedCreditStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedCreditStatus {
    fn default() -> Self {
        Self::Failed
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryReceivedCreditsParamsLinkedFlows {
    /// The source flow type.
    pub source_flow_type: GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryReceivedCreditsParamsStatus {
    Failed,
    Succeeded,
}

impl GetTreasuryReceivedCreditsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for GetTreasuryReceivedCreditsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryReceivedCreditsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryReceivedCreditsParamsStatus {
    fn default() -> Self {
        Self::Failed
    }
}
/// Initiating payment method details for the object.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetails {
    /// The source type.
    #[serde(rename = "type")]
    pub type_: PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType,

    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<
        PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsUsBankAccount,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryReceivedCreditsParamsNetwork {
    Ach,
    UsDomesticWire,
}

impl PostTestHelpersTreasuryReceivedCreditsParamsNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for PostTestHelpersTreasuryReceivedCreditsParamsNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTestHelpersTreasuryReceivedCreditsParamsNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryReceivedCreditsParamsNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::Payout => "payout",
        }
    }
}

impl AsRef<str> for GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryReceivedCreditsParamsLinkedFlowsSourceFlowType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType {
    UsBankAccount,
}

impl PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}
/// Optional fields for `us_bank_account`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryReceivedCreditsParamsInitiatingPaymentMethodDetailsUsBankAccount {
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

pub fn get_treasury_received_credits(
    client: &crate::Client,
    params: GetTreasuryReceivedCreditsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryReceivedCredit>> {
    client.get_query("/treasury/received_credits", params)
}

pub fn get_treasury_received_credits_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryReceivedCreditsIdParams,
) -> crate::Response<crate::generated::TreasuryReceivedCredit> {
    client.get_query(&format!("/treasury/received_credits/{id}", id = id), params)
}

pub fn post_test_helpers_treasury_received_credits(
    client: &crate::Client,
    params: PostTestHelpersTreasuryReceivedCreditsParams,
) -> crate::Response<crate::generated::TreasuryReceivedCredit> {
    client.post_form("/test_helpers/treasury/received_credits", params)
}
