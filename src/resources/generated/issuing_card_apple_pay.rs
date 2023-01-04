#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardApplePay {
    /// Apple Pay Eligibility.
    pub eligible: bool,

    /// Reason the card is ineligible for Apple Pay.
    pub ineligible_reason: Option<IssuingCardApplePayIneligibleReason>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardApplePayIneligibleReason {
    MissingAgreement,
    MissingCardholderContact,
    UnsupportedRegion,
}

impl IssuingCardApplePayIneligibleReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MissingAgreement => "missing_agreement",
            Self::MissingCardholderContact => "missing_cardholder_contact",
            Self::UnsupportedRegion => "unsupported_region",
        }
    }
}

impl AsRef<str> for IssuingCardApplePayIneligibleReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardApplePayIneligibleReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardApplePayIneligibleReason {
    fn default() -> Self {
        Self::MissingAgreement
    }
}
