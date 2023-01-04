#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderRequirements {
    /// If `disabled_reason` is present, all cards will decline authorizations with `cardholder_verification_required` reason.
    pub disabled_reason: Option<IssuingCardholderRequirementsDisabledReason>,

    /// Array of fields that need to be collected in order to verify and re-enable the cardholder.
    pub past_due: Option<Vec<IssuingCardholderRequirementsPastDue>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsDisabledReason {
    Listed,
    #[serde(rename = "rejected.listed")]
    RejectedListed,
    UnderReview,
}

impl IssuingCardholderRequirementsDisabledReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Listed => "listed",
            Self::RejectedListed => "rejected.listed",
            Self::UnderReview => "under_review",
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsDisabledReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsDisabledReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardholderRequirementsDisabledReason {
    fn default() -> Self {
        Self::Listed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardholderRequirementsPastDue {
    #[serde(rename = "company.tax_id")]
    CompanyTaxId,
    #[serde(rename = "individual.dob.day")]
    IndividualDobDay,
    #[serde(rename = "individual.dob.month")]
    IndividualDobMonth,
    #[serde(rename = "individual.dob.year")]
    IndividualDobYear,
    #[serde(rename = "individual.first_name")]
    IndividualFirstName,
    #[serde(rename = "individual.last_name")]
    IndividualLastName,
    #[serde(rename = "individual.verification.document")]
    IndividualVerificationDocument,
}

impl IssuingCardholderRequirementsPastDue {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CompanyTaxId => "company.tax_id",
            Self::IndividualDobDay => "individual.dob.day",
            Self::IndividualDobMonth => "individual.dob.month",
            Self::IndividualDobYear => "individual.dob.year",
            Self::IndividualFirstName => "individual.first_name",
            Self::IndividualLastName => "individual.last_name",
            Self::IndividualVerificationDocument => "individual.verification.document",
        }
    }
}

impl AsRef<str> for IssuingCardholderRequirementsPastDue {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardholderRequirementsPastDue {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardholderRequirementsPastDue {
    fn default() -> Self {
        Self::CompanyTaxId
    }
}
