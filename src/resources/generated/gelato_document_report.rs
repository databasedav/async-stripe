/// Result from a document check.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GelatoDocumentReport {
    /// Address as it appears in the document.
    pub address: Option<crate::generated::Address>,

    /// Date of birth as it appears in the document.
    pub dob: Option<crate::generated::GelatoDataDocumentReportDateOfBirth>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<crate::generated::GelatoDocumentReportError>,

    /// Expiration date of the document.
    pub expiration_date: Option<crate::generated::GelatoDataDocumentReportExpirationDate>,

    /// Array of [File](https://stripe.com/docs/api/files) ids containing images for this document.
    pub files: Option<Vec<String>>,

    /// First name as it appears in the document.
    pub first_name: Option<String>,

    /// Issued date of the document.
    pub issued_date: Option<crate::generated::GelatoDataDocumentReportIssuedDate>,

    /// Issuing country of the document.
    pub issuing_country: Option<String>,

    /// Last name as it appears in the document.
    pub last_name: Option<String>,

    /// Document ID number.
    pub number: Option<String>,

    /// Status of this `document` check.
    pub status: GelatoDocumentReportStatus,

    /// Type of the document.
    #[serde(rename = "type")]
    pub type_: Option<GelatoDocumentReportType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportStatus {
    Unverified,
    Verified,
}

impl GelatoDocumentReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoDocumentReportStatus {
    fn default() -> Self {
        Self::Unverified
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoDocumentReportType {
    DrivingLicense,
    IdCard,
    Passport,
}

impl GelatoDocumentReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for GelatoDocumentReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoDocumentReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoDocumentReportType {
    fn default() -> Self {
        Self::DrivingLicense
    }
}
