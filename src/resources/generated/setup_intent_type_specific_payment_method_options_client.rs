#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SetupIntentTypeSpecificPaymentMethodOptionsClient {
    /// Bank account verification method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SetupIntentTypeSpecificPaymentMethodOptionsClientVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}
