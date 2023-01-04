/// This is an object representing a file hosted on Stripe's servers.
///
/// The file may have been uploaded by yourself using the [create file](https://stripe.com/docs/api#create_file) request (for example, when uploading dispute evidence) or it may have been created by Stripe (for example, the results of a [Sigma scheduled query](#scheduled_queries)).  Related guide: [File Upload Guide](https://stripe.com/docs/file-upload).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct File {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The time at which the file expires and is no longer available in epoch seconds.
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A filename for the file, suitable for saving to a filesystem.
    pub filename: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// A list of [file links](https://stripe.com/docs/api#file_links) that point at this file.
    #[serde(default)]
    pub links: crate::params::List<crate::generated::FileLink>,

    /// The [purpose](https://stripe.com/docs/file-upload#uploading-a-file) of the uploaded file.
    pub purpose: FilePurpose,

    /// The size in bytes of the file object.
    pub size: u64,

    /// A user friendly title for the document.
    pub title: Option<String>,

    /// The type of the file returned (e.g., `csv`, `pdf`, `jpg`, or `png`).
    #[serde(rename = "type")]
    pub type_: Option<String>,

    /// The URL from which the file can be downloaded using your live secret API key.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFilesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub purpose: Option<GetFilesParamsPurpose>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFilesFileParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FilePurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    IdentityDocument,
    IdentityDocumentDownloadable,
    PciDocument,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalReaderSplashscreen,
}

impl FilePurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountRequirement => "account_requirement",
            Self::AdditionalVerification => "additional_verification",
            Self::BusinessIcon => "business_icon",
            Self::BusinessLogo => "business_logo",
            Self::CustomerSignature => "customer_signature",
            Self::DisputeEvidence => "dispute_evidence",
            Self::DocumentProviderIdentityDocument => "document_provider_identity_document",
            Self::FinanceReportRun => "finance_report_run",
            Self::IdentityDocument => "identity_document",
            Self::IdentityDocumentDownloadable => "identity_document_downloadable",
            Self::PciDocument => "pci_document",
            Self::Selfie => "selfie",
            Self::SigmaScheduledQuery => "sigma_scheduled_query",
            Self::TaxDocumentUserUpload => "tax_document_user_upload",
            Self::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
        }
    }
}

impl AsRef<str> for FilePurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FilePurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FilePurpose {
    fn default() -> Self {
        Self::AccountRequirement
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetFilesParamsPurpose {
    AccountRequirement,
    AdditionalVerification,
    BusinessIcon,
    BusinessLogo,
    CustomerSignature,
    DisputeEvidence,
    DocumentProviderIdentityDocument,
    FinanceReportRun,
    IdentityDocument,
    IdentityDocumentDownloadable,
    PciDocument,
    Selfie,
    SigmaScheduledQuery,
    TaxDocumentUserUpload,
    TerminalReaderSplashscreen,
}

impl GetFilesParamsPurpose {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountRequirement => "account_requirement",
            Self::AdditionalVerification => "additional_verification",
            Self::BusinessIcon => "business_icon",
            Self::BusinessLogo => "business_logo",
            Self::CustomerSignature => "customer_signature",
            Self::DisputeEvidence => "dispute_evidence",
            Self::DocumentProviderIdentityDocument => "document_provider_identity_document",
            Self::FinanceReportRun => "finance_report_run",
            Self::IdentityDocument => "identity_document",
            Self::IdentityDocumentDownloadable => "identity_document_downloadable",
            Self::PciDocument => "pci_document",
            Self::Selfie => "selfie",
            Self::SigmaScheduledQuery => "sigma_scheduled_query",
            Self::TaxDocumentUserUpload => "tax_document_user_upload",
            Self::TerminalReaderSplashscreen => "terminal_reader_splashscreen",
        }
    }
}

impl AsRef<str> for GetFilesParamsPurpose {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetFilesParamsPurpose {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetFilesParamsPurpose {
    fn default() -> Self {
        Self::AccountRequirement
    }
}
pub fn get_files(
    client: &crate::Client,
    params: GetFilesParams,
) -> crate::Response<crate::params::List<crate::generated::File>> {
    client.get_query("/files", params)
}

pub fn get_files_file(
    client: &crate::Client,
    file: String,
    params: GetFilesFileParams,
) -> crate::Response<crate::generated::File> {
    client.get_query(&format!("/files/{file}", file = file), params)
}
