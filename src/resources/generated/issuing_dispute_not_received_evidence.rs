#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeNotReceivedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Date when the cardholder expected to receive the product.
    pub expected_at: Option<crate::params::Timestamp>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeNotReceivedEvidenceProductType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeNotReceivedEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeNotReceivedEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeNotReceivedEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeNotReceivedEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeNotReceivedEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}
