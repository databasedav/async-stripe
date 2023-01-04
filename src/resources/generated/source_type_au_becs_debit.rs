#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceTypeAuBecsDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bsb_number: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,
}
