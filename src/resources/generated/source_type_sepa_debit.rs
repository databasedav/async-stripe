#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct SourceTypeSepaDebit {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch_code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fingerprint: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_reference: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_url: Option<String>,
}
