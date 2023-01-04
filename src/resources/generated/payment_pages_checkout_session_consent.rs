#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionConsent {
    /// If `opt_in`, the customer consents to receiving promotional communications
    /// from the merchant about this Checkout Session.
    pub promotions: Option<PaymentPagesCheckoutSessionConsentPromotions>,

    /// If `accepted`, the customer in this Checkout Session has agreed to the merchant's terms of service.
    pub terms_of_service: Option<PaymentPagesCheckoutSessionConsentTermsOfService>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentPromotions {
    OptIn,
    OptOut,
}

impl PaymentPagesCheckoutSessionConsentPromotions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OptIn => "opt_in",
            Self::OptOut => "opt_out",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentPromotions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentPromotions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentPagesCheckoutSessionConsentPromotions {
    fn default() -> Self {
        Self::OptIn
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentPagesCheckoutSessionConsentTermsOfService {
    Accepted,
}

impl PaymentPagesCheckoutSessionConsentTermsOfService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
        }
    }
}

impl AsRef<str> for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentPagesCheckoutSessionConsentTermsOfService {
    fn default() -> Self {
        Self::Accepted
    }
}
