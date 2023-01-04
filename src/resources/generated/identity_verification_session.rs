/// A VerificationSession guides you through the process of collecting and verifying the identities
/// of your users.
///
/// It contains details about the type of verification, such as what [verification check](/docs/identity/verification-checks) to perform.
/// Only create one VerificationSession for each verification in your system.  A VerificationSession transitions through [multiple statuses](/docs/identity/how-sessions-work) throughout its lifetime as it progresses through the verification flow.
/// The VerificationSession contains the user's verified data after verification checks are complete.  Related guide: [The Verification Sessions API](https://stripe.com/docs/identity/verification-sessions).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IdentityVerificationSession {
    /// The short-lived client secret used by Stripe.js to [show a verification modal](https://stripe.com/docs/js/identity/modal) inside your app.
    ///
    /// This client secret expires after 24 hours and can only be used once.
    /// Don’t store it, log it, embed it in a URL, or expose it to anyone other than the user.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs on [passing the client secret to the frontend](https://stripe.com/docs/identity/verification-sessions#client-secret) to learn more.
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// If present, this property tells you the last error encountered when processing the verification.
    pub last_error: Option<crate::generated::GelatoSessionLastError>,

    /// ID of the most recent VerificationReport.
    ///
    /// [Learn more about accessing detailed verification results.](https://stripe.com/docs/identity/verification-sessions#results).
    pub last_verification_report: Option<Vec<crate::generated::IdentityVerificationReport>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    pub options: crate::generated::GelatoVerificationSessionOptions,

    /// Redaction status of this VerificationSession.
    ///
    /// If the VerificationSession is not redacted, this field will be null.
    pub redaction: Option<crate::generated::VerificationSessionRedaction>,

    /// Status of this VerificationSession.
    ///
    /// [Learn more about the lifecycle of sessions](https://stripe.com/docs/identity/how-sessions-work).
    pub status: IdentityVerificationSessionStatus,

    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: IdentityVerificationSessionType,

    /// The short-lived URL that you use to redirect a user to Stripe to submit their identity information.
    ///
    /// This URL expires after 48 hours and can only be used once.
    /// Don’t store it, log it, send it in emails or expose it to anyone other than the user.
    /// Refer to our docs on [verifying identity documents](https://stripe.com/docs/identity/verify-identity-documents?platform=web&type=redirect) to learn how to redirect users to Stripe.
    pub url: Option<String>,

    /// The user’s verified data.
    pub verified_outputs: Option<crate::generated::GelatoVerifiedOutputs>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<PostIdentityVerificationSessionsParamsOptions>,

    /// The URL that the user will be redirected to upon completing the verification flow.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    pub type_: PostIdentityVerificationSessionsParamsType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIdentityVerificationSessionsSessionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIdentityVerificationSessionsParams {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetIdentityVerificationSessionsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsSessionCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsSessionRedactParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsSessionParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// A set of options for the session’s verification checks.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub options: Option<PostIdentityVerificationSessionsSessionParamsOptions>,

    /// The type of [verification check](https://stripe.com/docs/identity/verification-checks) to be performed.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostIdentityVerificationSessionsSessionParamsType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl IdentityVerificationSessionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Processing => "processing",
            Self::RequiresInput => "requires_input",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for IdentityVerificationSessionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationSessionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IdentityVerificationSessionStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IdentityVerificationSessionType {
    Document,
    IdNumber,
}

impl IdentityVerificationSessionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for IdentityVerificationSessionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IdentityVerificationSessionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IdentityVerificationSessionType {
    fn default() -> Self {
        Self::Document
    }
}
/// A set of options for the session’s verification checks.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsParamsOptions {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PostIdentityVerificationSessionsParamsOptionsDocument>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIdentityVerificationSessionsParamsType {
    Document,
    IdNumber,
}

impl PostIdentityVerificationSessionsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for PostIdentityVerificationSessionsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIdentityVerificationSessionsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIdentityVerificationSessionsParamsType {
    fn default() -> Self {
        Self::Document
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIdentityVerificationSessionsParamsStatus {
    Canceled,
    Processing,
    RequiresInput,
    Verified,
}

impl GetIdentityVerificationSessionsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Processing => "processing",
            Self::RequiresInput => "requires_input",
            Self::Verified => "verified",
        }
    }
}

impl AsRef<str> for GetIdentityVerificationSessionsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIdentityVerificationSessionsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIdentityVerificationSessionsParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// A set of options for the session’s verification checks.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsSessionParamsOptions {
    /// Options that apply to the [document check](https://stripe.com/docs/identity/verification-checks?type=document).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub document: Option<PostIdentityVerificationSessionsSessionParamsOptionsDocument>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIdentityVerificationSessionsSessionParamsType {
    Document,
    IdNumber,
}

impl PostIdentityVerificationSessionsSessionParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Document => "document",
            Self::IdNumber => "id_number",
        }
    }
}

impl AsRef<str> for PostIdentityVerificationSessionsSessionParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIdentityVerificationSessionsSessionParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIdentityVerificationSessionsSessionParamsType {
    fn default() -> Self {
        Self::Document
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsParamsOptionsDocument {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types:
        Option<Vec<PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes>>,

    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,

    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,

    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    ///
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIdentityVerificationSessionsSessionParamsOptionsDocument {
    /// Array of strings of allowed identity document types.
    ///
    /// If the provided identity document isn’t one of the allowed types, the verification check will fail with a document_type_not_allowed error code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_types:
        Option<Vec<PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes>>,

    /// Collect an ID number and perform an [ID number check](https://stripe.com/docs/identity/verification-checks?type=id-number) with the document’s extracted name and date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_id_number: Option<bool>,

    /// Disable image uploads, identity document images have to be captured using the device’s camera.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_live_capture: Option<bool>,

    /// Capture a face image and perform a [selfie check](https://stripe.com/docs/identity/verification-checks?type=selfie) comparing a photo ID and a picture of your user’s face.
    ///
    /// [Learn more](https://stripe.com/docs/identity/selfie).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_matching_selfie: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIdentityVerificationSessionsParamsOptionsDocumentAllowedTypes {
    fn default() -> Self {
        Self::DrivingLicense
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes {
    DrivingLicense,
    IdCard,
    Passport,
}

impl PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DrivingLicense => "driving_license",
            Self::IdCard => "id_card",
            Self::Passport => "passport",
        }
    }
}

impl AsRef<str> for PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIdentityVerificationSessionsSessionParamsOptionsDocumentAllowedTypes {
    fn default() -> Self {
        Self::DrivingLicense
    }
}
pub fn post_identity_verification_sessions(
    client: &crate::Client,
    params: PostIdentityVerificationSessionsParams,
) -> crate::Response<crate::generated::IdentityVerificationSession> {
    client.post_form("/identity/verification_sessions", params)
}

pub fn get_identity_verification_sessions_session(
    client: &crate::Client,
    session: String,
    params: GetIdentityVerificationSessionsSessionParams,
) -> crate::Response<crate::generated::IdentityVerificationSession> {
    client
        .get_query(&format!("/identity/verification_sessions/{session}", session = session), params)
}

pub fn get_identity_verification_sessions(
    client: &crate::Client,
    params: GetIdentityVerificationSessionsParams,
) -> crate::Response<crate::params::List<crate::generated::IdentityVerificationSession>> {
    client.get_query("/identity/verification_sessions", params)
}

pub fn post_identity_verification_sessions_session_cancel(
    client: &crate::Client,
    session: String,
    params: PostIdentityVerificationSessionsSessionCancelParams,
) -> crate::Response<crate::generated::IdentityVerificationSession> {
    client.post_form(
        &format!("/identity/verification_sessions/{session}/cancel", session = session),
        params,
    )
}

pub fn post_identity_verification_sessions_session_redact(
    client: &crate::Client,
    session: String,
    params: PostIdentityVerificationSessionsSessionRedactParams,
) -> crate::Response<crate::generated::IdentityVerificationSession> {
    client.post_form(
        &format!("/identity/verification_sessions/{session}/redact", session = session),
        params,
    )
}

pub fn post_identity_verification_sessions_session(
    client: &crate::Client,
    session: String,
    params: PostIdentityVerificationSessionsSessionParams,
) -> crate::Response<crate::generated::IdentityVerificationSession> {
    client
        .post_form(&format!("/identity/verification_sessions/{session}", session = session), params)
}
