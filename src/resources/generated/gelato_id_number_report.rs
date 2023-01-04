/// Result from an id_number check.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GelatoIdNumberReport {
    /// Date of birth.
    pub dob: Option<crate::generated::GelatoDataIdNumberReportDate>,

    /// Details on the verification error.
    ///
    /// Present when status is `unverified`.
    pub error: Option<crate::generated::GelatoIdNumberReportError>,

    /// First name.
    pub first_name: Option<String>,

    /// ID number.
    pub id_number: Option<String>,

    /// Type of ID number.
    pub id_number_type: Option<GelatoIdNumberReportIdNumberType>,

    /// Last name.
    pub last_name: Option<String>,

    /// Status of this `id_number` check.
    pub status: GelatoIdNumberReportStatus,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoIdNumberReportIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrCpf => "br_cpf",
            Self::SgNric => "sg_nric",
            Self::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoIdNumberReportIdNumberType {
    fn default() -> Self {
        Self::BrCpf
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoIdNumberReportStatus {
    Unverified,
    Verified,
}

impl GelatoIdNumberReportStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Unverified => "unverified",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for GelatoIdNumberReportStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoIdNumberReportStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoIdNumberReportStatus {
    fn default() -> Self {
        Self::Unverified
    }
}
