#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoDocumentReportError {
    /// A short machine-readable string giving the reason for the verification failure.
    pub code: Option<GelatoDocumentReportErrorCode>,

    /// A human-readable message giving the reason for the failure.
    ///
    /// These messages can be shown to your users.
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportErrorCode {
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
}

impl GelatoDocumentReportErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DocumentExpired => "document_expired",
            Self::DocumentTypeNotSupported => "document_type_not_supported",
            Self::DocumentUnverifiedOther => "document_unverified_other",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoDocumentReportErrorCode {
    fn default() -> Self {
        Self::DocumentExpired
    }
}
