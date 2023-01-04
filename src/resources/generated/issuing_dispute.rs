/// As a [card issuer](https://stripe.com/docs/issuing), you can dispute transactions that the cardholder does not recognize, suspects to be fraudulent, or has other issues with.
///
/// Related guide: [Disputing Transactions](https://stripe.com/docs/issuing/purchases/disputes).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingDispute {
    /// Disputed amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// Usually the amount of the `transaction`, but can differ (usually because of currency fluctuation).
    pub amount: i64,

    /// List of balance transactions associated with the dispute.
    pub balance_transactions: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The currency the `transaction` was made in.
    pub currency: crate::currency::Currency,

    pub evidence: crate::generated::IssuingDisputeEvidence,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// Current status of the dispute.
    pub status: IssuingDisputeStatus,

    /// The transaction being disputed.
    pub transaction: Vec<crate::generated::IssuingTransaction>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this dispute if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<crate::generated::IssuingDisputeTreasury>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingDisputesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetIssuingDisputesParamsStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParams {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// If not set, defaults to the full transaction amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PostIssuingDisputesParamsEvidence>,

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

    /// The ID of the issuing transaction to create a dispute for.
    ///
    /// For transaction on Treasury FinancialAccounts, use `treasury.received_debit`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,

    /// Params for disputes related to Treasury FinancialAccounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<PostIssuingDisputesParamsTreasury>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParams {
    /// The dispute amount in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Evidence provided for the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub evidence: Option<PostIssuingDisputesDisputeParamsEvidence>,

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
pub struct GetIssuingDisputesDisputeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeSubmitParams {
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
pub enum IssuingDisputeStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}

impl IssuingDisputeStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Submitted => "submitted",
            Self::Unsubmitted => "unsubmitted",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for IssuingDisputeStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeStatus {
    fn default() -> Self {
        Self::Expired
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIssuingDisputesParamsStatus {
    Expired,
    Lost,
    Submitted,
    Unsubmitted,
    Won,
}

impl GetIssuingDisputesParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Submitted => "submitted",
            Self::Unsubmitted => "unsubmitted",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for GetIssuingDisputesParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIssuingDisputesParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIssuingDisputesParamsStatus {
    fn default() -> Self {
        Self::Expired
    }
}
/// Evidence provided for the dispute.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidence {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<PostIssuingDisputesParamsEvidenceCanceled>,

    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<PostIssuingDisputesParamsEvidenceDuplicate>,

    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<PostIssuingDisputesParamsEvidenceFraudulent>,

    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribed>,

    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<PostIssuingDisputesParamsEvidenceNotReceived>,

    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<PostIssuingDisputesParamsEvidenceOther>,

    /// The reason for filing the dispute.
    ///
    /// The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PostIssuingDisputesParamsEvidenceReason>,

    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described: Option<PostIssuingDisputesParamsEvidenceServiceNotAsDescribed>,
}

/// Params for disputes related to Treasury FinancialAccounts.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsTreasury {
    /// The ID of the ReceivedDebit to initiate an Issuings dispute for.
    pub received_debit: String,
}

/// Evidence provided for the dispute.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidence {
    /// Evidence provided when `reason` is 'canceled'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<PostIssuingDisputesDisputeParamsEvidenceCanceled>,

    /// Evidence provided when `reason` is 'duplicate'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<PostIssuingDisputesDisputeParamsEvidenceDuplicate>,

    /// Evidence provided when `reason` is 'fraudulent'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<PostIssuingDisputesDisputeParamsEvidenceFraudulent>,

    /// Evidence provided when `reason` is 'merchandise_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribed>,

    /// Evidence provided when `reason` is 'not_received'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<PostIssuingDisputesDisputeParamsEvidenceNotReceived>,

    /// Evidence provided when `reason` is 'other'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<PostIssuingDisputesDisputeParamsEvidenceOther>,

    /// The reason for filing the dispute.
    ///
    /// The evidence should be submitted in the field of the same name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PostIssuingDisputesDisputeParamsEvidenceReason>,

    /// Evidence provided when `reason` is 'service_not_as_described'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described:
        Option<PostIssuingDisputesDisputeParamsEvidenceServiceNotAsDescribed>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceCanceled {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesParamsEvidenceCanceledProductType>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<PostIssuingDisputesParamsEvidenceCanceledReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceDuplicate {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    ///
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceFraudulent {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<crate::params::Timestamp>,

    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status:
        Option<PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceNotReceived {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesParamsEvidenceNotReceivedProductType>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceOther {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesParamsEvidenceOtherProductType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl PostIssuingDisputesParamsEvidenceReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            Self::NotReceived => "not_received",
            Self::Other => "other",
            Self::ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceReason {
    fn default() -> Self {
        Self::Canceled
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesParamsEvidenceServiceNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceCanceled {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Whether the cardholder was provided with a cancellation policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_policy_provided: Option<bool>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesDisputeParamsEvidenceCanceledProductType>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status: Option<PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceDuplicate {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the card statement showing that the product had already been paid for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_statement: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Copy of the receipt showing that the product had been paid for in cash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_receipt: Option<String>,

    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Image of the front and back of the check that was used to pay for the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub check_image: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Transaction (e.g., ipi_...) that the disputed transaction is a duplicate of.
    ///
    /// Of the two or more transactions that are copies of each other, this is original undisputed one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_transaction: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceFraudulent {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<crate::params::Timestamp>,

    /// Description of the cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_description: Option<String>,

    /// Result of cardholder's attempt to return the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_status:
        Option<PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceNotReceived {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when the cardholder expected to receive the product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceOther {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_type: Option<PostIssuingDisputesDisputeParamsEvidenceOtherProductType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl PostIssuingDisputesDisputeParamsEvidenceReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::MerchandiseNotAsDescribed => "merchandise_not_as_described",
            Self::NotReceived => "not_received",
            Self::Other => "other",
            Self::ServiceNotAsDescribed => "service_not_as_described",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesDisputeParamsEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceReason {
    fn default() -> Self {
        Self::Canceled
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingDisputesDisputeParamsEvidenceServiceNotAsDescribed {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub additional_documentation: Option<String>,

    /// Date when order was canceled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Reason for canceling the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub explanation: Option<String>,

    /// Date when the product was received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceCanceledProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesParamsEvidenceCanceledProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceCanceledProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceCanceledProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceCanceledReturnStatus {
    MerchantRejected,
    Successful,
}

impl PostIssuingDisputesParamsEvidenceCanceledReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceCanceledReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceCanceledReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}

impl PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceNotReceivedProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesParamsEvidenceNotReceivedProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceNotReceivedProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceNotReceivedProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesParamsEvidenceOtherProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesParamsEvidenceOtherProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesParamsEvidenceOtherProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesParamsEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesParamsEvidenceOtherProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceCanceledProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesDisputeParamsEvidenceCanceledProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceCanceledProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesDisputeParamsEvidenceCanceledProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceCanceledProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus {
    MerchantRejected,
    Successful,
}

impl PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceCanceledReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    MerchantRejected,
    Successful,
}

impl PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceMerchandiseNotAsDescribedReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceNotReceivedProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingDisputesDisputeParamsEvidenceOtherProductType {
    Merchandise,
    Service,
}

impl PostIssuingDisputesDisputeParamsEvidenceOtherProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for PostIssuingDisputesDisputeParamsEvidenceOtherProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingDisputesDisputeParamsEvidenceOtherProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingDisputesDisputeParamsEvidenceOtherProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}
pub fn get_issuing_disputes(
    client: &crate::Client,
    params: GetIssuingDisputesParams,
) -> crate::Response<crate::params::List<crate::generated::IssuingDispute>> {
    client.get_query("/issuing/disputes", params)
}

pub fn post_issuing_disputes(
    client: &crate::Client,
    params: PostIssuingDisputesParams,
) -> crate::Response<crate::generated::IssuingDispute> {
    client.post_form("/issuing/disputes", params)
}

pub fn post_issuing_disputes_dispute(
    client: &crate::Client,
    dispute: String,
    params: PostIssuingDisputesDisputeParams,
) -> crate::Response<crate::generated::IssuingDispute> {
    client.post_form(&format!("/issuing/disputes/{dispute}", dispute = dispute), params)
}

pub fn get_issuing_disputes_dispute(
    client: &crate::Client,
    dispute: String,
    params: GetIssuingDisputesDisputeParams,
) -> crate::Response<crate::generated::IssuingDispute> {
    client.get_query(&format!("/issuing/disputes/{dispute}", dispute = dispute), params)
}

pub fn post_issuing_disputes_dispute_submit(
    client: &crate::Client,
    dispute: String,
    params: PostIssuingDisputesDisputeSubmitParams,
) -> crate::Response<crate::generated::IssuingDispute> {
    client.post_form(&format!("/issuing/disputes/{dispute}/submit", dispute = dispute), params)
}
