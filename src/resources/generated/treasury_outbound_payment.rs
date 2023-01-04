/// Use OutboundPayments to send funds to another party's external bank account or [FinancialAccount](https://stripe.com/docs/api#financial_accounts).
///
/// To send money to an account belonging to the same user, use an [OutboundTransfer](https://stripe.com/docs/api#outbound_transfers).  Simulate OutboundPayment state changes with the `/v1/test_helpers/treasury/outbound_payments` endpoints.
/// These methods can only be called on test mode objects.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryOutboundPayment {
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

    /// ID of the [customer](https://stripe.com/docs/api/customers) to whom an OutboundPayment is sent.
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The PaymentMethod via which an OutboundPayment is sent.
    ///
    /// This field can be empty if the OutboundPayment was created using `destination_payment_method_data`.
    pub destination_payment_method: Option<String>,

    /// Details about the PaymentMethod for an OutboundPayment.
    pub destination_payment_method_details:
        Option<crate::generated::OutboundPaymentsPaymentMethodDetails>,

    /// Details about the end user.
    pub end_user_details: Option<
        crate::generated::TreasuryOutboundPaymentsResourceOutboundPaymentResourceEndUserDetails,
    >,

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

    /// Details about a returned OutboundPayment.
    ///
    /// Only set when the status is `returned`.
    pub returned_details: Option<crate::generated::TreasuryOutboundPaymentsResourceReturnedStatus>,

    /// The description that appears on the receiving end for an OutboundPayment (for example, bank statement for external bank transfer).
    pub statement_descriptor: String,

    /// Current status of the OutboundPayment: `processing`, `failed`, `posted`, `returned`, `canceled`.
    ///
    /// An OutboundPayment is `processing` if it has been created and is pending.
    /// The status changes to `posted` once the OutboundPayment has been "confirmed" and funds have left the account, or to `failed` or `canceled`.
    /// If an OutboundPayment fails to arrive at its destination, its status will change to `returned`.
    pub status: TreasuryOutboundPaymentStatus,

    pub status_transitions:
        crate::generated::TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Vec<crate::generated::TreasuryTransaction>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParams {
    /// Amount (in cents) to be transferred.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the customer to whom the OutboundPayment is sent.
    ///
    /// Must match the Customer attached to the `destination_payment_method` passed in.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The PaymentMethod to use as the payment instrument for the OutboundPayment.
    ///
    /// Exclusive with `destination_payment_method_data`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method: Option<String>,

    /// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
    ///
    /// Exclusive with `destination_payment_method`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_data:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodData>,

    /// Payment method-specific configuration for this OutboundPayment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment_method_options:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptions>,

    /// End user details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_user_details: Option<PostTreasuryOutboundPaymentsParamsEndUserDetails>,

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

    /// The description that appears on the receiving end for this OutboundPayment (for example, bank statement for external bank transfer).
    ///
    /// Maximum 10 characters for `ach` payments, 140 characters for `wire` payments, or 500 characters for `stripe` network transfers.
    /// The default value is `payment`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryOutboundPaymentsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryOutboundPaymentsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

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
    pub status: Option<GetTreasuryOutboundPaymentsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsIdCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundPaymentsIdFailParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundPaymentsIdPostParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundPaymentsIdReturnParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Optional hash to set the the return code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_details:
        Option<PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetails>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryOutboundPaymentStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl TreasuryOutboundPaymentStatus {
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

impl AsRef<str> for TreasuryOutboundPaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryOutboundPaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryOutboundPaymentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Hash used to generate the PaymentMethod to be used for this OutboundPayment.
///
/// Exclusive with `destination_payment_method`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodData {
    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataBillingDetails>,

    /// Required if type is set to `financial_account`.
    ///
    /// The FinancialAccount ID to send funds to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType,

    /// Required hash if type is set to `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccount>,
}

/// Payment method-specific configuration for this OutboundPayment.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptions {
    /// Optional fields for `us_bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccount>,
}

/// End user details.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsEndUserDetails {
    /// IP address of the user initiating the OutboundPayment.
    ///
    /// Must be supplied if `present` is set to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// `True` if the OutboundPayment creation request is being made on behalf of an end user by a platform.
    ///
    /// Otherwise, `false`.
    pub present: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryOutboundPaymentsParamsStatus {
    Canceled,
    Failed,
    Posted,
    Processing,
    Returned,
}

impl GetTreasuryOutboundPaymentsParamsStatus {
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

impl AsRef<str> for GetTreasuryOutboundPaymentsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryOutboundPaymentsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryOutboundPaymentsParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Optional hash to set the the return code.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetails {
    /// The return code to be set on the OutboundPayment object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode>,
}

/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address:
        Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataBillingDetailsAddress>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType {
    FinancialAccount,
    UsBankAccount,
}

impl PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialAccount => "financial_account",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataType {
    fn default() -> Self {
        Self::FinancialAccount
    }
}
/// Required hash if type is set to `us_bank_account`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccount {
    /// Account holder type: individual or company.
#[serde(skip_serializing_if = "Option::is_none")]
pub account_holder_type: Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType>,

    /// Account number of the bank account.
#[serde(skip_serializing_if = "Option::is_none")]
pub account_number: Option<String>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
#[serde(skip_serializing_if = "Option::is_none")]
pub account_type: Option<PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType>,

    /// The ID of a Financial Connections Account to use as a payment method.
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_connections_account: Option<String>,

    /// Routing number of the bank account.
#[serde(skip_serializing_if = "Option::is_none")]
pub routing_number: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccount {
    /// The US bank account network that must be used for this OutboundPayment.
    ///
    /// If not set, we will default to the PaymentMethod's preferred network.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network: Option<
        PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode {
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

impl PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode {
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

impl AsRef<str> for PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTestHelpersTreasuryOutboundPaymentsIdReturnParamsReturnedDetailsCode {
    fn default() -> Self {
        Self::AccountClosed
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    Company,
    Individual,
}

impl PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str>
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountHolderType
{
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str>
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodDataUsBankAccountAccountType
{
    fn default() -> Self {
        Self::Checking
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    Ach,
    UsDomesticWire,
}

impl PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str>
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryOutboundPaymentsParamsDestinationPaymentMethodOptionsUsBankAccountNetwork
{
    fn default() -> Self {
        Self::Ach
    }
}
pub fn post_treasury_outbound_payments(
    client: &crate::Client,
    params: PostTreasuryOutboundPaymentsParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client.post_form("/treasury/outbound_payments", params)
}

pub fn get_treasury_outbound_payments_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryOutboundPaymentsIdParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client.get_query(&format!("/treasury/outbound_payments/{id}", id = id), params)
}

pub fn get_treasury_outbound_payments(
    client: &crate::Client,
    params: GetTreasuryOutboundPaymentsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryOutboundPayment>> {
    client.get_query("/treasury/outbound_payments", params)
}

pub fn post_treasury_outbound_payments_id_cancel(
    client: &crate::Client,
    id: String,
    params: PostTreasuryOutboundPaymentsIdCancelParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client.post_form(&format!("/treasury/outbound_payments/{id}/cancel", id = id), params)
}

pub fn post_test_helpers_treasury_outbound_payments_id_fail(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryOutboundPaymentsIdFailParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client
        .post_form(&format!("/test_helpers/treasury/outbound_payments/{id}/fail", id = id), params)
}

pub fn post_test_helpers_treasury_outbound_payments_id_post(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryOutboundPaymentsIdPostParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client
        .post_form(&format!("/test_helpers/treasury/outbound_payments/{id}/post", id = id), params)
}

pub fn post_test_helpers_treasury_outbound_payments_id_return(
    client: &crate::Client,
    id: String,
    params: PostTestHelpersTreasuryOutboundPaymentsIdReturnParams,
) -> crate::Response<crate::generated::TreasuryOutboundPayment> {
    client.post_form(
        &format!("/test_helpers/treasury/outbound_payments/{id}/return", id = id),
        params,
    )
}
