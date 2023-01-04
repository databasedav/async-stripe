#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeEvidence {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled: Option<crate::generated::IssuingDisputeCanceledEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub duplicate: Option<crate::generated::IssuingDisputeDuplicateEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fraudulent: Option<crate::generated::IssuingDisputeFraudulentEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub merchandise_not_as_described:
        Option<crate::generated::IssuingDisputeMerchandiseNotAsDescribedEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub not_received: Option<crate::generated::IssuingDisputeNotReceivedEvidence>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub other: Option<crate::generated::IssuingDisputeOtherEvidence>,

    /// The reason for filing the dispute.
    ///
    /// Its value will match the field containing the evidence.
    pub reason: IssuingDisputeEvidenceReason,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_not_as_described:
        Option<crate::generated::IssuingDisputeServiceNotAsDescribedEvidence>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingDisputeEvidenceReason {
    Canceled,
    Duplicate,
    Fraudulent,
    MerchandiseNotAsDescribed,
    NotReceived,
    Other,
    ServiceNotAsDescribed,
}

impl IssuingDisputeEvidenceReason {
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

impl AsRef<str> for IssuingDisputeEvidenceReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingDisputeEvidenceReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingDisputeEvidenceReason {
    fn default() -> Self {
        Self::Canceled
    }
}
