/// Use [InboundTransfers](https://stripe.com/docs/treasury/moving-money/financial-accounts/into/inbound-transfers) to add funds to your [FinancialAccount](https://stripe.com/docs/api#financial_accounts) via a PaymentMethod that is owned by you.
///
/// The funds will be transferred via an ACH debit.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryInboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Returns `true` if the InboundTransfer is able to be canceled.
    pub cancelable: bool,

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

    /// Details about this InboundTransfer's failure.
    ///
    /// Only set when status is `failed`.
    pub failure_details: Option<crate::generated::TreasuryInboundTransfersResourceFailureDetails>,

    /// The FinancialAccount that received the funds.
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    pub linked_flows:
        crate::generated::TreasuryInboundTransfersResourceInboundTransferResourceLinkedFlows,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The origin payment method to be debited for an InboundTransfer.
    pub origin_payment_method: String,

    /// Details about the PaymentMethod for an InboundTransfer.
    pub origin_payment_method_details: Option<crate::generated::InboundTransfers>,

    /// Returns `true` if the funds for an InboundTransfer were returned after the InboundTransfer went to the `succeeded` state.
    pub returned: Option<bool>,

    /// Statement descriptor shown when funds are debited from the source.
    ///
    /// Not all payment networks support `statement_descriptor`.
    pub statement_descriptor: String,

    /// Status of the InboundTransfer: `processing`, `succeeded`, `failed`, and `canceled`.
    ///
    /// An InboundTransfer is `processing` if it is created and pending.
    /// The status changes to `succeeded` once the funds have been "confirmed" and a `transaction` is created and posted.
    /// The status changes to `failed` if the transfer fails.
    pub status: TreasuryInboundTransferStatus,

    pub status_transitions:
        crate::generated::TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Option<Vec<crate::generated::TreasuryTransaction>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryInboundTransfersInboundTransferCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryInboundTransfersParams {
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

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The origin payment method to be debited for the InboundTransfer.
    pub origin_payment_method: String,

    /// The complete description that appears on your customers' statements.
    ///
    /// Maximum 10 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryInboundTransfersIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryInboundTransfersParams {
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
    pub status: Option<GetTreasuryInboundTransfersParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryInboundTransfersIdSucceedParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryInboundTransfersIdFailParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Details about a failed InboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub failure_details: Option<PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetails>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryInboundTransfersIdReturnParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryInboundTransferStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl TreasuryInboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryInboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryInboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryInboundTransferStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryInboundTransfersParamsStatus {
    Canceled,
    Failed,
    Processing,
    Succeeded,
}

impl GetTreasuryInboundTransfersParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for GetTreasuryInboundTransfersParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryInboundTransfersParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryInboundTransfersParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Details about a failed InboundTransfer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetails {
    /// Reason for the failure.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    DebitNotAuthorized,
    IncorrectAccountHolderAddress,
    IncorrectAccountHolderName,
    IncorrectAccountHolderTaxId,
    InsufficientFunds,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::DebitNotAuthorized => "debit_not_authorized",
            Self::IncorrectAccountHolderAddress => "incorrect_account_holder_address",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::IncorrectAccountHolderTaxId => "incorrect_account_holder_tax_id",
            Self::InsufficientFunds => "insufficient_funds",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryInboundTransfersIdFailParamsFailureDetailsCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}
pub fn post_treasury_inbound_transfers_inbound_transfer_cancel(
    client: &crate::Client,
    inbound_transfer: String,
    params: PostTreasuryInboundTransfersInboundTransferCancelParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client.post_form(
        &format!(
            "/treasury/inbound_transfers/{inbound_transfer}/cancel",
            inbound_transfer = inbound_transfer
        ),
        params,
    )
}

pub fn post_treasury_inbound_transfers(
    client: &crate::Client,
    params: PostTreasuryInboundTransfersParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client.post_form("/treasury/inbound_transfers", params)
}

pub fn get_treasury_inbound_transfers_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryInboundTransfersIdParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client.get_query(&format!("/treasury/inbound_transfers/{id}", id = id), params)
}

pub fn get_treasury_inbound_transfers(
    client: &crate::Client,
    params: GetTreasuryInboundTransfersParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryInboundTransfer>> {
    client.get_query("/treasury/inbound_transfers", params)
}

pub fn post_test_helpers_treasury_inbound_transfers_id_succeed(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryInboundTransfersIdSucceedParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client.post_form(
        &format!("/test_helpers/treasury/inbound_transfers/{id}/succeed", id = id),
        params,
    )
}

pub fn post_test_helpers_treasury_inbound_transfers_id_fail(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryInboundTransfersIdFailParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client
        .post_form(&format!("/test_helpers/treasury/inbound_transfers/{id}/fail", id = id), params)
}

pub fn post_test_helpers_treasury_inbound_transfers_id_return(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryInboundTransfersIdReturnParams,
) -> crate::Response<crate::generated::TreasuryInboundTransfer> {
    client.post_form(
        &format!("/test_helpers/treasury/inbound_transfers/{id}/return", id = id),
        params,
    )
}
