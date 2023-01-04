/// A VerificationReport is the result of an attempt to collect and verify data from a user.
/// The collection of verification checks performed is determined from the `type` and `options`
/// parameters used.
///
/// You can find the result of each verification check performed in the appropriate sub-resource: `document`, `id_number`, `selfie`.  Each VerificationReport contains a copy of any data collected by the user as well as reference IDs which can be used to access collected images through the [FileUpload](https://stripe.com/docs/api/files) API.
/// To configure and create VerificationReports, use the [VerificationSession](https://stripe.com/docs/api/identity/verification_sessions) API.  Related guides: [Accessing verification results](https://stripe.com/docs/identity/verification-sessions#results).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IdentityVerificationReport {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<crate::generated::GelatoDocumentReport>,

    /// Unique identifier for the object.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub id_number: Option<crate::generated::GelatoIdNumberReport>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub options: crate::generated::GelatoVerificationReportOptions,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub selfie: Option<crate::generated::GelatoSelfieReport>,

    /// Type of report.
    #[serde(rename = "type")]
    pub type_: IdentityVerificationReportType,

    /// ID of the VerificationSession that created this report.
    pub verification_session: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIdentityVerificationReportsReportParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIdentityVerificationReportsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GetIdentityVerificationReportsParamsType>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_session: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationReportType {
    Document,
    IdNumber,
}

impl IdentityVerificationReportType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for IdentityVerificationReportType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationReportType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IdentityVerificationReportType {
    fn default() -> Self {
        Self::Document
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIdentityVerificationReportsParamsType {
    Document,
    IdNumber,
}

impl GetIdentityVerificationReportsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for GetIdentityVerificationReportsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIdentityVerificationReportsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIdentityVerificationReportsParamsType {
    fn default() -> Self {
        Self::Document
    }
}
pub fn get_identity_verification_reports_report(
    client: &crate::Client,
    report: String,
    params: GetIdentityVerificationReportsReportParams,
) -> crate::Response<crate::generated::IdentityVerificationReport> {
    client.get_query(&format!("/identity/verification_reports/{report}", report = report), params)
}

pub fn get_identity_verification_reports(
    client: &crate::Client,
    params: GetIdentityVerificationReportsParams,
) -> crate::Response<crate::params::List<crate::generated::IdentityVerificationReport>> {
    client.get_query("/identity/verification_reports", params)
}
