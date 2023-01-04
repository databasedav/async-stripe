/// A dispute occurs when a customer questions your charge with their card issuer.
/// When this happens, you're given the opportunity to respond to the dispute with
/// evidence that shows that the charge is legitimate.
///
/// You can find more information about the dispute process in our [Disputes and Fraud](/docs/disputes) documentation.  Related guide: [Disputes and Fraud](https://stripe.com/docs/disputes).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Dispute {
    /// Disputed amount.
    ///
    /// Usually the amount of the charge, but can differ (usually because of currency fluctuation or because only part of the order is disputed).
    pub amount: i64,

    /// List of zero, one, or two balance transactions that show funds withdrawn and reinstated to your Stripe account as a result of this dispute.
    pub balance_transactions: Vec<crate::generated::BalanceTransaction>,

    /// ID of the charge that was disputed.
    pub charge: Vec<crate::generated::Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    pub evidence: crate::generated::DisputeEvidence,

    pub evidence_details: crate::generated::DisputeEvidenceDetails,

    /// Unique identifier for the object.
    pub id: String,

    /// If true, it is still possible to refund the disputed payment.
    ///
    /// Once the payment has been fully refunded, no further funds will be withdrawn from your Stripe account as a result of this dispute.
    pub is_charge_refundable: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// Network-dependent reason code for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub network_reason_code: Option<String>,

    /// ID of the PaymentIntent that was disputed.
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,

    /// Reason given by cardholder for dispute.
    ///
    /// Possible values are `bank_cannot_process`, `check_returned`, `credit_not_processed`, `customer_initiated`, `debit_not_authorized`, `duplicate`, `fraudulent`, `general`, `incorrect_account_details`, `insufficient_funds`, `product_not_received`, `product_unacceptable`, `subscription_canceled`, or `unrecognized`.
    /// Read more about [dispute reasons](https://stripe.com/docs/disputes/categories).
    pub reason: String,

    /// Current status of dispute.
    ///
    /// Possible values are `warning_needs_response`, `warning_under_review`, `warning_closed`, `needs_response`, `under_review`, `charge_refunded`, `won`, or `lost`.
    pub status: DisputeStatus,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetDisputesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetDisputesDisputeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostDisputesDisputeParams {
    /// Evidence to upload, to respond to a dispute.
    ///
    /// Updating any field in the hash will submit all fields in the hash for review.
    /// The combined character count of all fields is limited to 150,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PostDisputesDisputeParamsEvidence>,

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

    /// Whether to immediately submit evidence to the bank.
    ///
    /// If `false`, evidence is staged on the dispute.
    /// Staged evidence is visible in the API and Dashboard, and can be submitted to the bank by making another request with this attribute set to `true` (the default).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub submit: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostDisputesDisputeCloseParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum DisputeStatus {
    ChargeRefunded,
    Lost,
    NeedsResponse,
    UnderReview,
    WarningClosed,
    WarningNeedsResponse,
    WarningUnderReview,
    Won,
}

impl DisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeRefunded => "charge_refunded",
            Self::Lost => "lost",
            Self::NeedsResponse => "needs_response",
            Self::UnderReview => "under_review",
            Self::WarningClosed => "warning_closed",
            Self::WarningNeedsResponse => "warning_needs_response",
            Self::WarningUnderReview => "warning_under_review",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for DisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for DisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for DisputeStatus {
    fn default() -> Self {
        Self::ChargeRefunded
    }
}
/// Evidence to upload, to respond to a dispute.
///
/// Updating any field in the hash will submit all fields in the hash for review.
/// The combined character count of all fields is limited to 150,000.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostDisputesDisputeParamsEvidence {
    /// Any server or activity logs showing proof that the customer accessed or downloaded the purchased digital product.
    ///
    /// This information should include IP addresses, corresponding timestamps, and any detailed recorded activity.
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub access_activity_log: Option<String>,

    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_address: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your subscription cancellation policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy: Option<String>,

    /// An explanation of how and when the customer was shown your refund policy prior to purchase.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_disclosure: Option<String>,

    /// A justification for why the customer's subscription was not canceled.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_rebuttal: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any communication with the customer that you feel is relevant to your case.
    ///
    /// Examples include emails proving that the customer received the product or service, or demonstrating their use of or satisfaction with the product or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_communication: Option<String>,

    /// The email address of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_email_address: Option<String>,

    /// The name of the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_name: Option<String>,

    /// The IP address that the customer used when making the purchase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_purchase_ip: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) A relevant document or contract showing the customer's signature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_signature: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation for the prior charge that can uniquely identify the charge, such as a receipt, shipping label, work order, etc.
    ///
    /// This document should be paired with a similar document from the disputed payment that proves the two payments are separate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_documentation: Option<String>,

    /// An explanation of the difference between the disputed charge versus the prior charge that appears to be a duplicate.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_explanation: Option<String>,

    /// The Stripe ID for the prior charge which appears to be a duplicate of the disputed charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate_charge_id: Option<String>,

    /// A description of the product or service that was sold.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any receipt or message sent to the customer notifying them of the charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receipt: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Your refund policy, as shown to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy: Option<String>,

    /// Documentation demonstrating that the customer was shown your refund policy prior to purchase.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_policy_disclosure: Option<String>,

    /// A justification for why the customer is not entitled to a refund.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_refusal_explanation: Option<String>,

    /// The date on which the customer received or began receiving the purchased service, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_date: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a service was provided to the customer.
    ///
    /// This could include a copy of a signed contract, work order, or other form of written agreement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_documentation: Option<String>,

    /// The address to which a physical product was shipped.
    ///
    /// You should try to include as complete address information as possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_address: Option<String>,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    ///
    /// If multiple carriers were used for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_carrier: Option<String>,

    /// The date on which a physical product began its route to the shipping address, in a clear human-readable format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_date: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Documentation showing proof that a product was shipped to the customer at the same address the customer provided to you.
    ///
    /// This could include a copy of the shipment receipt, shipping label, etc.
    /// It should show the customer's full shipping address, if possible.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_documentation: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_tracking_number: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Any additional evidence or statements.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_file: Option<String>,

    /// Any additional evidence or statements.
    ///
    /// Has a maximum character count of 20,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub uncategorized_text: Option<String>,
}

pub fn get_disputes(
    client: &crate::Client,
    params: GetDisputesParams,
) -> crate::Response<crate::params::List<crate::generated::Dispute>> {
    client.get_query("/disputes", params)
}

pub fn get_disputes_dispute(
    client: &crate::Client,
    dispute: String,
    params: GetDisputesDisputeParams,
) -> crate::Response<crate::generated::Dispute> {
    client.get_query(&format!("/disputes/{dispute}", dispute = dispute), params)
}

pub fn post_disputes_dispute(
    client: &crate::Client,
    dispute: String,
    params: PostDisputesDisputeParams,
) -> crate::Response<crate::generated::Dispute> {
    client.post_form(&format!("/disputes/{dispute}", dispute = dispute), params)
}

pub fn post_disputes_dispute_close(
    client: &crate::Client,
    dispute: String,
    params: PostDisputesDisputeCloseParams,
) -> crate::Response<crate::generated::Dispute> {
    client.post_form(&format!("/disputes/{dispute}/close", dispute = dispute), params)
}
