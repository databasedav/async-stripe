#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentProcessing {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::PaymentIntentCardProcessing>,

    /// Type of the payment method for which payment is in `processing` state, one of `card`.
    #[serde(rename = "type")]
    pub type_: PaymentIntentProcessingType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentIntentProcessingType {
    Card,
}

impl PaymentIntentProcessingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Card => "card",
        }
    }
}

impl AsRef<str> for PaymentIntentProcessingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentIntentProcessingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentIntentProcessingType {
    fn default() -> Self {
        Self::Card
    }
}
