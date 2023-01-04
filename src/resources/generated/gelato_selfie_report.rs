/// Result from a selfie check.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GelatoSelfieReport {
    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the identity document used in this check.
    pub document: Option<String>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<crate::generated::GelatoSelfieReportError>,

    /// ID of the [File](https://stripe.com/docs/api/files) holding the image of the selfie used in this check.
    pub selfie: Option<String>,

    /// Status of this `selfie` check.
    pub status: GelatoSelfieReportStatus,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoSelfieReportStatus {
    Unverified,
    Verified,
}

impl GelatoSelfieReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoSelfieReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoSelfieReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoSelfieReportStatus {
    fn default() -> Self {
        Self::Unverified
    }
}
