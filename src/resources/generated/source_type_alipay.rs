#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceTypeAlipay {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_string: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub native_url: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}