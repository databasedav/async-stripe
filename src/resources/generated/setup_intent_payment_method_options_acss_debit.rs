#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentPaymentMethodOptionsAcssDebit {
    /// Currency supported by the bank account.
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<crate::generated::SetupIntentPaymentMethodOptionsMandateOptionsAcssDebit>,

    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SetupIntentPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}
