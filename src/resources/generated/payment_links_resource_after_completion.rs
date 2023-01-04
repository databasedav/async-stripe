#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceAfterCompletion {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_confirmation:
        Option<crate::generated::PaymentLinksResourceCompletionBehaviorConfirmationPage>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<crate::generated::PaymentLinksResourceCompletionBehaviorRedirect>,

    /// The specified behavior after the purchase is complete.
    #[serde(rename = "type")]
    pub type_: PaymentLinksResourceAfterCompletionType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentLinksResourceAfterCompletionType {
    HostedConfirmation,
    Redirect,
}

impl PaymentLinksResourceAfterCompletionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::HostedConfirmation => "hosted_confirmation",
            Self::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for PaymentLinksResourceAfterCompletionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentLinksResourceAfterCompletionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentLinksResourceAfterCompletionType {
    fn default() -> Self {
        Self::HostedConfirmation
    }
}
