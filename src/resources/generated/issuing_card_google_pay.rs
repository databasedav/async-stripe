#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardGooglePay {
    /// Google Pay Eligibility.
    pub eligible: bool,

    /// Reason the card is ineligible for Google Pay.
    pub ineligible_reason: Option<IssuingCardGooglePayIneligibleReason>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardGooglePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl IssuingCardGooglePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingAgreement => "missing_agreement",
            Self::MissingCardholderContact => "missing_cardholder_contact",
            Self::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl AsRef<str> for IssuingCardGooglePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardGooglePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardGooglePayIneligibleReason {
    fn default() -> Self {
        Self::MissingAgreement
    }
}
