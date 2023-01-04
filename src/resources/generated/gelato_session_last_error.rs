/// Shows last VerificationSession error.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoSessionLastError {
    /// A short machine-readable string giving the reason for the verification or user-session failure.
    pub code: Option<GelatoSessionLastErrorCode>,

    /// A message that explains the reason for verification or user-session failure.
    pub reason: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSessionLastErrorCode {
    Abandoned,
    ConsentDeclined,
    CountryNotSupported,
    DeviceNotSupported,
    DocumentExpired,
    DocumentTypeNotSupported,
    DocumentUnverifiedOther,
    IdNumberInsufficientDocumentData,
    IdNumberMismatch,
    IdNumberUnverifiedOther,
    SelfieDocumentMissingPhoto,
    SelfieFaceMismatch,
    SelfieManipulated,
    SelfieUnverifiedOther,
    UnderSupportedAge,
}

impl GelatoSessionLastErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abandoned => "abandoned",
            Self::ConsentDeclined => "consent_declined",
            Self::CountryNotSupported => "country_not_supported",
            Self::DeviceNotSupported => "device_not_supported",
            Self::DocumentExpired => "document_expired",
            Self::DocumentTypeNotSupported => "document_type_not_supported",
            Self::DocumentUnverifiedOther => "document_unverified_other",
            Self::IdNumberInsufficientDocumentData => "id_number_insufficient_document_data",
            Self::IdNumberMismatch => "id_number_mismatch",
            Self::IdNumberUnverifiedOther => "id_number_unverified_other",
            Self::SelfieDocumentMissingPhoto => "selfie_document_missing_photo",
            Self::SelfieFaceMismatch => "selfie_face_mismatch",
            Self::SelfieManipulated => "selfie_manipulated",
            Self::SelfieUnverifiedOther => "selfie_unverified_other",
            Self::UnderSupportedAge => "under_supported_age",
        }
    }
}

impl AsRef<str> for GelatoSessionLastErrorCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSessionLastErrorCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoSessionLastErrorCode {
    fn default() -> Self {
        Self::Abandoned
    }
}
