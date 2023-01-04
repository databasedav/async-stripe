/// To share the contents of a `File` object with non-Stripe users, you can
/// create a `FileLink`.
///
/// `FileLink`s contain a URL that can be used to retrieve the contents of the file without authentication.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FileLink {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Whether this link is already expired.
    pub expired: bool,

    /// Time at which the link expires.
    pub expires_at: Option<crate::resources::Scheduled>,

    /// The file object this link points to.
    pub file: Vec<crate::generated::File>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The publicly accessible URL to download the file.
    pub url: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFileLinksLinkParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostFileLinksParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A future timestamp after which the link will no longer be usable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// The ID of the file.
    ///
    /// The file's `purpose` must be one of the following: `business_icon`, `business_logo`, `customer_signature`, `dispute_evidence`, `finance_report_run`, `identity_document_downloadable`, `pci_document`, `selfie`, `sigma_scheduled_query`, `tax_document_user_upload`, or `terminal_reader_splashscreen`.
    pub file: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostFileLinksLinkParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A future timestamp after which the link will no longer be usable, or `now` to expire the link immediately.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetFileLinksParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expired: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub file: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

pub fn get_file_links_link(
    client: &crate::Client,
    link: String,
    params: GetFileLinksLinkParams,
) -> crate::Response<crate::generated::FileLink> {
    client.get_query(&format!("/file_links/{link}", link = link), params)
}

pub fn post_file_links(
    client: &crate::Client,
    params: PostFileLinksParams,
) -> crate::Response<crate::generated::FileLink> {
    client.post_form("/file_links", params)
}

pub fn post_file_links_link(
    client: &crate::Client,
    link: String,
    params: PostFileLinksLinkParams,
) -> crate::Response<crate::generated::FileLink> {
    client.post_form(&format!("/file_links/{link}", link = link), params)
}

pub fn get_file_links(
    client: &crate::Client,
    params: GetFileLinksParams,
) -> crate::Response<crate::params::List<crate::generated::FileLink>> {
    client.get_query("/file_links", params)
}
