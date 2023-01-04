/// Represents a reader action to set the reader display.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalReaderReaderResourceSetReaderDisplayAction {
    /// Cart object to be displayed by the reader.
    pub cart: Option<crate::generated::TerminalReaderReaderResourceCart>,

    /// Type of information to be displayed by the reader.
    #[serde(rename = "type")]
    pub type_: TerminalReaderReaderResourceSetReaderDisplayActionType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TerminalReaderReaderResourceSetReaderDisplayActionType {
    Cart,
}

impl TerminalReaderReaderResourceSetReaderDisplayActionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cart => "cart",
        }
    }
}

impl AsRef<str> for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TerminalReaderReaderResourceSetReaderDisplayActionType {
    fn default() -> Self {
        Self::Cart
    }
}
