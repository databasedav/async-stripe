#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceTypeIdeal {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bic: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,
}
