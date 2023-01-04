#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceTypeGiropay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_name: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
