/// Use OutboundTransfers to transfer funds from a [FinancialAccount](https://stripe.com/docs/api#financial_accounts) to a PaymentMethod belonging to the same entity.
///
/// To send funds to a different party, use [OutboundPayments](https://stripe.com/docs/api#outbound_payments) instead.
/// You can send funds over ACH rails or through a domestic wire transfer to a user's own external bank account.  Simulate OutboundTransfer state changes with the `/v1/test_helpers/treasury/outbound_transfers` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryOutboundTransfer {
    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Returns `true` if the object can be canceled, and `false` otherwise.
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

    /// The PaymentMethod used as the payment instrument for an OutboundTransfer.
    pub destination_payment_method: Option<String>,

    pub destination_payment_method_details: crate::generated::OutboundTransfersPaymentMethodDetails,

    /// The date when funds are expected to arrive in the destination account.
    pub expected_arrival_date: crate::params::Timestamp,

    /// The FinancialAccount that funds were pulled from.
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// Details about a returned OutboundTransfer.
    ///
    /// Only set when the status is `returned`.
    pub returned_details:
        Option<crate::generated::TreasuryOutboundTransfersResourceReturnedDetails>,

    /// Information about the OutboundTransfer to be sent to the recipient account.
    pub statement_descriptor: String,

    /// Current status of the OutboundTransfer: `processing`, `failed`, `canceled`, `posted`, `returned`.
    ///
    /// An OutboundTransfer is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundTransfer has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundTransfer fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundTransferStatus,

    pub status_transitions: crate::generated::TreasuryOutboundTransfersResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Vec<crate::generated::TreasuryTransaction>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundTransfersParams {
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

    /// The PaymentMethod to use as the payment instrument for the OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,

    /// Hash describing payment method configuration details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options:
        Option<PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptions>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The FinancialAccount to pull funds from.
    pub financial_account: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Statement descriptor to be shown on the receiving end of an OutboundTransfer.
    ///
    /// Maximum 10 characters for `ach` transfers or 140 characters for `wire` transfers.
    /// The default value is `transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryOutboundTransfersOutboundTransferParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryOutboundTransfersParams {
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
    pub status: Option<GetTreasuryOutboundTransfersParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundTransfersOutboundTransferCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Details about a returned OutboundTransfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details:
        Option<PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetails>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundTransferStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundTransferStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Posted => "posted",
            Self::Processing => "processing",
            Self::Returned => "returned",
        }
    }
}

impl AsRef<str> for TreasuryOutboundTransferStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundTransferStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryOutboundTransferStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Hash describing payment method configuration details.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryOutboundTransfersParamsStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl GetTreasuryOutboundTransfersParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Failed => "failed",
            Self::Posted => "posted",
            Self::Processing => "processing",
            Self::Returned => "returned",
        }
    }
}

impl AsRef<str> for GetTreasuryOutboundTransfersParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryOutboundTransfersParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryOutboundTransfersParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Details about a returned OutboundTransfer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetails {
    /// Reason for the return.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<
        PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccount {
    /// Designate the OutboundTransfer as using a US bank account network configuration.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<
        PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode {
    AccountClosed,
    AccountFrozen,
    BankAccountRestricted,
    BankOwnershipChanged,
    Declined,
    IncorrectAccountHolderName,
    InvalidAccountNumber,
    InvalidCurrency,
    NoAccount,
    Other,
}

impl PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountClosed => "account_closed",
            Self::AccountFrozen => "account_frozen",
            Self::BankAccountRestricted => "bank_account_restricted",
            Self::BankOwnershipChanged => "bank_ownership_changed",
            Self::Declined => "declined",
            Self::IncorrectAccountHolderName => "incorrect_account_holder_name",
            Self::InvalidAccountNumber => "invalid_account_number",
            Self::InvalidCurrency => "invalid_currency",
            Self::NoAccount => "no_account",
            Self::Other => "other",
        }
    }
}

impl AsRef<str>
    for PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParamsReturnedDetailsCode
{
    fn default() -> Self {
        Self::AccountClosed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str>
    for PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryOutboundTransfersParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn default() -> Self {
        Self::Ach
    }
}
pub fn post_treasury_outbound_transfers(
    client: &crate::Client,
    params: PostTreasuryOutboundTransfersParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.post_form("/treasury/outbound_transfers", params)
}

pub fn get_treasury_outbound_transfers_outbound_transfer(
    client: &crate::Client,
    outbound_transfer: String,
    params: GetTreasuryOutboundTransfersOutboundTransferParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.get_query(
        &format!(
            "/treasury/outbound_transfers/{outbound_transfer}",
            outbound_transfer = outbound_transfer
        ),
        params,
    )
}

pub fn get_treasury_outbound_transfers(
    client: &crate::Client,
    params: GetTreasuryOutboundTransfersParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryOutboundTransfer>> {
    client.get_query("/treasury/outbound_transfers", params)
}

pub fn post_treasury_outbound_transfers_outbound_transfer_cancel(
    client: &crate::Client,
    outbound_transfer: String,
    params: PostTreasuryOutboundTransfersOutboundTransferCancelParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.post_form(
        &format!(
            "/treasury/outbound_transfers/{outbound_transfer}/cancel",
            outbound_transfer = outbound_transfer
        ),
        params,
    )
}

pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_fail(
    client: &crate::Client,
    outbound_transfer: String,
    params: PostTestHelpersTreasuryOutboundTransfersOutboundTransferFailParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.post_form(
        &format!(
            "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/fail",
            outbound_transfer = outbound_transfer
        ),
        params,
    )
}

pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_post(
    client: &crate::Client,
    outbound_transfer: String,
    params: PostTestHelpersTreasuryOutboundTransfersOutboundTransferPostParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.post_form(
        &format!(
            "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/post",
            outbound_transfer = outbound_transfer
        ),
        params,
    )
}

pub fn post_test_helpers_treasury_outbound_transfers_outbound_transfer_return(
    client: &crate::Client,
    outbound_transfer: String,
    params: PostTestHelpersTreasuryOutboundTransfersOutboundTransferReturnParams,
) -> crate::Response<crate::generated::TreasuryOutboundTransfer> {
    client.post_form(
        &format!(
            "/test_helpers/treasury/outbound_transfers/{outbound_transfer}/return",
            outbound_transfer = outbound_transfer
        ),
        params,
    )
}
