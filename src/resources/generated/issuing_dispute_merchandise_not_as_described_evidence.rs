#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeMerchandiseNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Date when the product was received.
    pub received_at: Option<crate::params::Timestamp>,

    /// Description of the cardholder's attempt to return the product.
    pub return_description: Option<String>,

    /// Result of cardholder's attempt to return the product.
    pub return_status: Option<IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus>,

    /// Date when the product was returned or attempted to be returned.
    pub returned_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    MerchantRejected,
    Successful,
}

impl IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MerchantRejected => "merchant_rejected",
            Self::Successful => "successful",
        }
    }
}

impl AsRef<str> for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeMerchandiseNotAsDescribedEvidenceReturnStatus {
    fn default() -> Self {
        Self::MerchantRejected
    }
}
