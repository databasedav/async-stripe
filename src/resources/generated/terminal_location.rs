/// A Location represents a grouping of readers.
///
/// Related guide: [Fleet Management](https://stripe.com/docs/terminal/fleet/locations).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalLocation {
    pub address: crate::generated::Address,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,

    /// The display name of the location.
    pub display_name: String,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedGetTerminalLocationsLocation {
    TerminalLocation(crate::generated::TerminalLocation),
    DeletedTerminalLocation(crate::generated::DeletedTerminalLocation),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTerminalLocationsLocationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalLocationsParams {
    /// The full address of the location.
    pub address: PostTerminalLocationsParamsAddress,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,

    /// A name for the location.
    pub display_name: String,

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
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedPostTerminalLocationsLocation {
    TerminalLocation(crate::generated::TerminalLocation),
    DeletedTerminalLocation(crate::generated::DeletedTerminalLocation),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalLocationsLocationParams {
    /// The full address of the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostTerminalLocationsLocationParamsAddress>,

    /// The ID of a configuration that will be used to customize all readers in this location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub configuration_overrides: Option<String>,

    /// A name for the location.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub display_name: Option<String>,

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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTerminalLocationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

/// The full address of the location.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalLocationsParamsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// The full address of the location.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalLocationsLocationParamsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

pub fn get_terminal_locations_location(
    client: &crate::Client,
    location: String,
    params: GetTerminalLocationsLocationParams,
) -> crate::Response<ReturnedGetTerminalLocationsLocation> {
    client.get_query(&format!("/terminal/locations/{location}", location = location), params)
}

pub fn post_terminal_locations(
    client: &crate::Client,
    params: PostTerminalLocationsParams,
) -> crate::Response<crate::generated::TerminalLocation> {
    client.post_form("/terminal/locations", params)
}

pub fn post_terminal_locations_location(
    client: &crate::Client,
    location: String,
    params: PostTerminalLocationsLocationParams,
) -> crate::Response<ReturnedPostTerminalLocationsLocation> {
    client.post_form(&format!("/terminal/locations/{location}", location = location), params)
}

pub fn get_terminal_locations(
    client: &crate::Client,
    params: GetTerminalLocationsParams,
) -> crate::Response<crate::params::List<crate::generated::TerminalLocation>> {
    client.get_query("/terminal/locations", params)
}

pub fn delete_terminal_locations_location(
    client: &crate::Client,
    location: String,
) -> crate::Response<crate::generated::DeletedTerminalLocation> {
    client.delete(&format!("/terminal/locations/{location}", location = location))
}
