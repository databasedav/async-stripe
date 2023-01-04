/// Value lists allow you to group values together which can then be referenced in rules.
///
/// Related guide: [Default Stripe Lists](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RadarValueList {
    /// The name of the value list for use in rules.
    pub alias: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The name or email address of the user who created this value list.
    pub created_by: String,

    /// Unique identifier for the object.
    pub id: String,

    /// The type of items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    pub item_type: RadarValueListItemType,

    /// List of items contained within this value list.
    pub list_items: crate::params::List<crate::generated::RadarValueListItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The name of the value list.
    pub name: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetRadarValueListsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub contains: Option<String>,

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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetRadarValueListsValueListParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostRadarValueListsParams {
    /// The name of the value list for use in rules.
    pub alias: String,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Type of the items in the value list.
    ///
    /// One of `card_fingerprint`, `card_bin`, `email`, `ip_address`, `country`, `string`, `case_sensitive_string`, or `customer_id`.
    /// Use `string` if the item type is unknown or mixed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub item_type: Option<PostRadarValueListsParamsItemType>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The human-readable name of the value list.
    pub name: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostRadarValueListsValueListParams {
    /// The name of the value list for use in rules.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alias: Option<String>,

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

    /// The human-readable name of the value list.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum RadarValueListItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    String,
}

impl RadarValueListItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardBin => "card_bin",
            Self::CardFingerprint => "card_fingerprint",
            Self::CaseSensitiveString => "case_sensitive_string",
            Self::Country => "country",
            Self::CustomerId => "customer_id",
            Self::Email => "email",
            Self::IpAddress => "ip_address",
            Self::String => "string",
        }
    }
}

impl AsRef<str> for RadarValueListItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for RadarValueListItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for RadarValueListItemType {
    fn default() -> Self {
        Self::CardBin
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostRadarValueListsParamsItemType {
    CardBin,
    CardFingerprint,
    CaseSensitiveString,
    Country,
    CustomerId,
    Email,
    IpAddress,
    String,
}

impl PostRadarValueListsParamsItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardBin => "card_bin",
            Self::CardFingerprint => "card_fingerprint",
            Self::CaseSensitiveString => "case_sensitive_string",
            Self::Country => "country",
            Self::CustomerId => "customer_id",
            Self::Email => "email",
            Self::IpAddress => "ip_address",
            Self::String => "string",
        }
    }
}

impl AsRef<str> for PostRadarValueListsParamsItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostRadarValueListsParamsItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostRadarValueListsParamsItemType {
    fn default() -> Self {
        Self::CardBin
    }
}
pub fn get_radar_value_lists(
    client: &crate::Client,
    params: GetRadarValueListsParams,
) -> crate::Response<crate::params::List<crate::generated::RadarValueList>> {
    client.get_query("/radar/value_lists", params)
}

pub fn get_radar_value_lists_value_list(
    client: &crate::Client,
    value_list: String,
    params: GetRadarValueListsValueListParams,
) -> crate::Response<crate::generated::RadarValueList> {
    client.get_query(&format!("/radar/value_lists/{value_list}", value_list = value_list), params)
}

pub fn post_radar_value_lists(
    client: &crate::Client,
    params: PostRadarValueListsParams,
) -> crate::Response<crate::generated::RadarValueList> {
    client.post_form("/radar/value_lists", params)
}

pub fn post_radar_value_lists_value_list(
    client: &crate::Client,
    value_list: String,
    params: PostRadarValueListsValueListParams,
) -> crate::Response<crate::generated::RadarValueList> {
    client.post_form(&format!("/radar/value_lists/{value_list}", value_list = value_list), params)
}

pub fn delete_radar_value_lists_value_list(
    client: &crate::Client,
    value_list: String,
) -> crate::Response<crate::generated::DeletedRadarValueList> {
    client.delete(&format!("/radar/value_lists/{value_list}", value_list = value_list))
}
