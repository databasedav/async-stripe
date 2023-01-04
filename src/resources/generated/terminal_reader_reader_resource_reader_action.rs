/// Represents an action performed by the reader.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalReaderReaderResourceReaderAction {
    /// Failure code, only set if status is `failed`.
    pub failure_code: Option<String>,

    /// Detailed failure message, only set if status is `failed`.
    pub failure_message: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_payment_intent:
        Option<crate::generated::TerminalReaderReaderResourceProcessPaymentIntentAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub process_setup_intent:
        Option<crate::generated::TerminalReaderReaderResourceProcessSetupIntentAction>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub set_reader_display:
        Option<crate::generated::TerminalReaderReaderResourceSetReaderDisplayAction>,

    /// Status of the action performed by the reader.
    pub status: TerminalReaderReaderResourceReaderActionStatus,

    /// Type of action performed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceReaderActionType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionStatus {
    Failed,
    InProgress,
    Succeeded,
}

impl TerminalReaderReaderResourceReaderActionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::InProgress => "in_progress",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TerminalReaderReaderResourceReaderActionStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceReaderActionType {
    ProcessPaymentIntent,
    ProcessSetupIntent,
    SetReaderDisplay,
}

impl TerminalReaderReaderResourceReaderActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ProcessPaymentIntent => "process_payment_intent",
            Self::ProcessSetupIntent => "process_setup_intent",
            Self::SetReaderDisplay => "set_reader_display",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceReaderActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceReaderActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TerminalReaderReaderResourceReaderActionType {
    fn default() -> Self {
        Self::ProcessPaymentIntent
    }
}
