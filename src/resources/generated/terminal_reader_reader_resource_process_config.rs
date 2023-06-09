/// Represents a per-transaction override of a reader configuration.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TerminalReaderReaderResourceProcessConfig {
    /// Override showing a tipping selection screen on this transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub skip_tipping: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<crate::generated::TerminalReaderReaderResourceTippingConfig>,
}