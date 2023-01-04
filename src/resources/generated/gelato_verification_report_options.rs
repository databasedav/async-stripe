#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoVerificationReportOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<crate::generated::GelatoReportDocumentOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<crate::generated::GelatoReportIdNumberOptions>,
}
