#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeOtherEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Description of the merchandise or service that was purchased.
    pub product_description: Option<String>,

    /// Whether the product was a merchandise or service.
    pub product_type: Option<IssuingDisputeOtherEvidenceProductType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeOtherEvidenceProductType {
    Merchandise,
    Service,
}

impl IssuingDisputeOtherEvidenceProductType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Merchandise => "merchandise",
            Self::Service => "service",
        }
    }
}

impl AsRef<str> for IssuingDisputeOtherEvidenceProductType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeOtherEvidenceProductType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeOtherEvidenceProductType {
    fn default() -> Self {
        Self::Merchandise
    }
}
