#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LegalEntityCompanyVerification {
    pub document: crate::generated::LegalEntityCompanyVerificationDocument,
}
