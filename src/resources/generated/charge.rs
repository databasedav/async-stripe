/// To charge a credit or a debit card, you create a `Charge` object.
///
/// You can retrieve and refund individual charges as well as list all charges.
/// Charges are identified by a unique, random ID.  Related guide: [Accept a payment with the Charges API](https://stripe.com/docs/payments/accept-a-payment-charges).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Charge {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alternate_statement_descriptors: Option<crate::generated::AlternateStatementDescriptors>,

    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    pub amount: i64,

    /// Amount in %s captured (can be less than the amount attribute on the charge if a partial capture was made).
    pub amount_captured: i64,

    /// Amount in %s refunded (can be less than the amount attribute on the charge if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that created the charge.
    pub application: Option<Vec<crate::generated::Application>>,

    /// The application fee (if any) for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee: Option<Vec<crate::generated::ApplicationFee>>,

    /// The amount of the application fee (if any) requested for the charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees) for details.
    pub application_fee_amount: Option<i64>,

    /// Authorization code on the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub authorization_code: Option<String>,

    /// ID of the balance transaction that describes the impact of this charge on your account balance (not including refunds or disputes).
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    pub billing_details: crate::generated::BillingDetails,

    /// The full statement descriptor that is passed to card networks, and that is displayed on your customers' credit card and bank statements.
    ///
    /// Allows you to see what the statement descriptor looks like after the static and dynamic portions are combined.
    pub calculated_statement_descriptor: Option<String>,

    /// If the charge was created without capturing, this Boolean represents whether it is still uncaptured or has since been captured.
    pub captured: bool,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the customer this charge is for if one exists.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// ID of an existing, connected Stripe account to transfer funds to if `transfer_data` was specified in the charge request.
    pub destination: Option<Vec<crate::generated::Account>>,

    /// Details about the dispute if the charge has been disputed.
    pub dispute: Option<Vec<crate::generated::Dispute>>,

    /// Whether the charge has been disputed.
    pub disputed: bool,

    /// ID of the balance transaction that describes the reversal of the balance on your account due to payment failure.
    pub failure_balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Error code explaining reason for charge failure if available (see [the errors section](https://stripe.com/docs/api#errors) for a list of codes).
    pub failure_code: Option<String>,

    /// Message to user further explaining reason for charge failure if available.
    pub failure_message: Option<String>,

    /// Information on fraud assessments for the charge.
    pub fraud_details: Option<crate::generated::ChargeFraudDetails>,

    /// Unique identifier for the object.
    pub id: String,

    /// ID of the invoice this charge is for if one exists.
    pub invoice: Option<Vec<crate::generated::Invoice>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub level3: Option<crate::generated::Level3>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The account (if any) the charge was made on behalf of without triggering an automatic transfer.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers) for details.
    pub on_behalf_of: Option<Vec<crate::generated::Account>>,

    /// Details about whether the payment was accepted, and why.
    ///
    /// See [understanding declines](https://stripe.com/docs/declines) for details.
    pub outcome: Option<crate::generated::ChargeOutcome>,

    /// `true` if the charge succeeded, or was successfully authorized for later capture.
    pub paid: bool,

    /// ID of the PaymentIntent associated with this charge, if one exists.
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,

    /// ID of the payment method used in this charge.
    pub payment_method: Option<String>,

    /// Details about the payment method at the time of the transaction.
    pub payment_method_details: Option<crate::generated::PaymentMethodDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<crate::generated::RadarRadarOptions>,

    /// This is the email address that the receipt for this charge was sent to.
    pub receipt_email: Option<String>,

    /// This is the transaction number that appears on email receipts sent for this charge.
    ///
    /// This attribute will be `null` until a receipt has been sent.
    pub receipt_number: Option<String>,

    /// This is the URL to view the receipt for this charge.
    ///
    /// The receipt is kept up-to-date to the latest state of the charge, including any refunds.
    /// If the charge is for an Invoice, the receipt will be stylized as an Invoice receipt.
    pub receipt_url: Option<String>,

    /// Whether the charge has been fully refunded.
    ///
    /// If the charge is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the charge.
    #[serde(default)]
    pub refunds: crate::params::List<crate::generated::Refund>,

    /// ID of the review associated with this charge if one exists.
    pub review: Option<Vec<crate::generated::Review>>,

    /// Shipping information for the charge.
    pub shipping: Option<crate::generated::Shipping>,

    /// This is a legacy field that will be removed in the future.
    ///
    /// It contains the Source, Card, or BankAccount object used for the charge.
    /// For details about the payment method used for this charge, refer to `payment_method` or `payment_method_details` instead.
    pub source: Option<crate::generated::PaymentSource>,

    /// The transfer ID which created this charge.
    ///
    /// Only present if the charge came from another Stripe account.
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub source_transfer: Option<Vec<crate::generated::Transfer>>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,

    /// The status of the payment is either `succeeded`, `pending`, or `failed`.
    pub status: ChargeStatus,

    /// ID of the transfer to the `destination` account (only applicable if the charge was created using the `destination` parameter).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer: Option<Vec<crate::generated::Transfer>>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    pub transfer_data: Option<crate::generated::ChargeTransferData>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ReturnedGetChargesSearch {
    pub data: Vec<crate::generated::Charge>,

    pub has_more: bool,

    pub next_page: Option<String>,

    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReturnedGetChargesSearchObject,

    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,

    pub url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetChargesSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    pub query: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetChargesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParams {
    /// Amount intended to be collected by this payment.
    ///
    /// A positive integer representing how much to charge in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// The minimum amount is $0.50 US or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    /// The amount value supports up to eight digits (e.g., a value of 99999999 for a USD charge of $999,999.99).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,

    /// A fee in cents (or local equivalent) that will be applied to the charge and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the `Stripe-Account` header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/direct-charges#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Whether to immediately capture the charge.
    ///
    /// Defaults to `true`.
    /// When `false`, the charge issues an authorization (or pre-authorization), and will need to be [captured](https://stripe.com/docs/api#capture_charge) later.
    /// Uncaptured charges expire after a set number of days (7 by default).
    /// For more information, see the [authorizing charges and settling later](https://stripe.com/docs/charges/placing-a-hold) documentation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture: Option<bool>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The ID of an existing customer that will be charged in this request.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string which you can attach to a `Charge` object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<PostChargesParamsDestination>,

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

    /// The Stripe account ID for which these funds are intended.
    ///
    /// Automatically set if you use the `destination` parameter.
    /// For details, see [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/charges-transfers#on-behalf-of).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<PostChargesParamsRadarOptions>,

    /// The email address to which this charge's [receipt](https://stripe.com/docs/dashboard/receipts) will be sent.
    ///
    /// The receipt will not be sent until the charge is paid, and no receipts will be sent for test mode charges.
    /// If this charge is for a [Customer](https://stripe.com/docs/api/customers/object), the email address specified here will override the customer's email address.
    /// If `receipt_email` is specified for a charge in live mode, a receipt will be sent regardless of your [email settings](https://dashboard.stripe.com/account/emails).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// Shipping information for the charge.
    ///
    /// Helps prevent fraud on charges for physical goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostChargesParamsShipping>,

    /// A payment source to be charged.
    ///
    /// This can be the ID of a [card](https://stripe.com/docs/api#cards) (i.e., credit or debit card), a [bank account](https://stripe.com/docs/api#bank_accounts), a [source](https://stripe.com/docs/api#sources), a [token](https://stripe.com/docs/api#tokens), or a [connected account](https://stripe.com/docs/connect/account-debits#charging-a-connected-account).
    /// For certain sources---namely, [cards](https://stripe.com/docs/api#cards), [bank accounts](https://stripe.com/docs/api#bank_accounts), and attached [sources](https://stripe.com/docs/api#sources)---you must also pass the ID of the associated customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostChargesParamsTransferData>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// For details, see [Grouping transactions](https://stripe.com/docs/connect/charges-transfers#transfer-options).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetChargesChargeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeParams {
    /// The ID of an existing customer that will be associated with this request.
    ///
    /// This field may only be updated if there is no existing associated customer with this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string which you can attach to a charge object.
    ///
    /// It is displayed when in the web interface alongside the charge.
    /// Note that if you use Stripe to send automatic email receipts to your customers, your receipt emails will include the `description` of the charge(s) that they are describing.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A set of key-value pairs you can attach to a charge giving information about its riskiness.
    ///
    /// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
    /// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
    /// Stripe will use the information you send to improve our fraud detection algorithms.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraud_details: Option<PostChargesChargeParamsFraudDetails>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// This is the email address that the receipt for this charge will be sent to.
    ///
    /// If this field is updated, then a new email receipt will be sent to the updated address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// Shipping information for the charge.
    ///
    /// Helps prevent fraud on charges for physical goods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostChargesChargeParamsShipping>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeCaptureParams {
    /// The amount to capture, which must be less than or equal to the original amount.
    ///
    /// Any additional amount will be automatically refunded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An application fee to add on to this charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee: Option<i64>,

    /// An application fee amount to add on to this charge, which must be less than or equal to the original amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The email address to send this charge's receipt to.
    ///
    /// This will override the previously-specified email address for this charge, if one was set.
    /// Receipts will not be sent in test mode.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt_email: Option<String>,

    /// For card charges, use `statement_descriptor_suffix` instead.
    ///
    /// Otherwise, you can use this value as the complete description of a charge on your customers’ statements.
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about the charge that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// An optional dictionary including the account to automatically transfer to as part of a destination charge.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostChargesChargeCaptureParamsTransferData>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// `transfer_group` may only be provided if it has not been set.
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ChargeStatus {
    Failed,
    Pending,
    Succeeded,
}

impl ChargeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for ChargeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ChargeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ChargeStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnedGetChargesSearchObject {
    SearchResult,
}

impl ReturnedGetChargesSearchObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for ReturnedGetChargesSearchObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnedGetChargesSearchObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ReturnedGetChargesSearchObject {
    fn default() -> Self {
        Self::SearchResult
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParamsDestination {
    /// ID of an existing, connected Stripe account.
    pub account: String,

    /// The amount to transfer to the destination account without creating an `Application Fee` object.
    ///
    /// Cannot be combined with the `application_fee` parameter.
    /// Must be less than or equal to the charge amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParamsRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// Shipping information for the charge.
///
/// Helps prevent fraud on charges for physical goods.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParamsShipping {
    /// Shipping address.
    pub address: PostChargesParamsShippingAddress,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    pub name: String,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
///
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParamsTransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

/// A set of key-value pairs you can attach to a charge giving information about its riskiness.
///
/// If you believe a charge is fraudulent, include a `user_report` key with a value of `fraudulent`.
/// If you believe a charge is safe, include a `user_report` key with a value of `safe`.
/// Stripe will use the information you send to improve our fraud detection algorithms.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeParamsFraudDetails {
    /// Either `safe` or `fraudulent`.
    pub user_report: PostChargesChargeParamsFraudDetailsUserReport,
}

/// Shipping information for the charge.
///
/// Helps prevent fraud on charges for physical goods.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeParamsShipping {
    /// Shipping address.
    pub address: PostChargesChargeParamsShippingAddress,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    pub name: String,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// An optional dictionary including the account to automatically transfer to as part of a destination charge.
///
/// [See the Connect documentation](https://stripe.com/docs/connect/destination-charges) for details.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeCaptureParamsTransferData {
    /// The amount transferred to the destination account, if specified.
    ///
    /// By default, the entire charge amount is transferred to the destination account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,
}

/// Shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesParamsShippingAddress {
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
pub enum PostChargesChargeParamsFraudDetailsUserReport {
    Fraudulent,
    Safe,
}

impl PostChargesChargeParamsFraudDetailsUserReport {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Fraudulent => "fraudulent",
            Self::Safe => "safe",
        }
    }
}

impl AsRef<str> for PostChargesChargeParamsFraudDetailsUserReport {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostChargesChargeParamsFraudDetailsUserReport {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostChargesChargeParamsFraudDetailsUserReport {
    fn default() -> Self {
        Self::Fraudulent
    }
}
/// Shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostChargesChargeParamsShippingAddress {
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

pub fn get_charges_search(
    client: &crate::Client,
    params: GetChargesSearchParams,
) -> crate::Response<ReturnedGetChargesSearch> {
    client.get_query("/charges/search", params)
}

pub fn get_charges(
    client: &crate::Client,
    params: GetChargesParams,
) -> crate::Response<crate::params::List<crate::generated::Charge>> {
    client.get_query("/charges", params)
}

pub fn post_charges(
    client: &crate::Client,
    params: PostChargesParams,
) -> crate::Response<crate::generated::Charge> {
    client.post_form("/charges", params)
}

pub fn get_charges_charge(
    client: &crate::Client,
    charge: String,
    params: GetChargesChargeParams,
) -> crate::Response<crate::generated::Charge> {
    client.get_query(&format!("/charges/{charge}", charge = charge), params)
}

pub fn post_charges_charge(
    client: &crate::Client,
    charge: String,
    params: PostChargesChargeParams,
) -> crate::Response<crate::generated::Charge> {
    client.post_form(&format!("/charges/{charge}", charge = charge), params)
}

pub fn post_charges_charge_capture(
    client: &crate::Client,
    charge: String,
    params: PostChargesChargeCaptureParams,
) -> crate::Response<crate::generated::Charge> {
    client.post_form(&format!("/charges/{charge}/capture", charge = charge), params)
}
