#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoSelfieReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoSelfieReportErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportErrorCode {
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
}

impl GelatoSelfieReportErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            Self::SelfieFaceMismatch => "selfie_face_mismatch",
            Self::SelfieManipulated => "selfie_manipulated",
            Self::SelfieUnverifiedOther => "selfie_unverified_other",
        }
    }
}

impl AsRef<str> for GelatoSelfieReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoSelfieReportErrorCode {
    fn default() -> Self {
        Self::SelfieDocumentMissingPhoto
    }
}
