#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceMandateNotificationAcssDebitData {
    /// The statement descriptor associate with the debit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
