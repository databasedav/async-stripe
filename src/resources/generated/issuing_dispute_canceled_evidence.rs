#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeCanceledEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Date when order was canceled.
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Whether the cardholder was provided with a cancellation policy.
    pub cancellation_policy_provided: Option<bool>,

    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,

    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeCanceledEvidenceProductType>,

    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeCanceledEvidenceReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeCanceledEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeCanceledEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeCanceledEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeCanceledEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeCanceledEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

impl IssuingDisputeCanceledEvidenceReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for IssuingDisputeCanceledEvidenceReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeCanceledEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeCanceledEvidenceReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}
