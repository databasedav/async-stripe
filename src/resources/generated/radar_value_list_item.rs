/// Value list items allow you to add specific values to a given Radar value list, which can then be used in rules.
///
/// Related guide: [Managing List Items](https://stripe.com/docs/radar/lists#managing-list-items).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RadarValueListItem {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The name or email address of the user who added this item to the value list.
    pub created_by: String,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The value of the item.
    pub value: String,

    /// The identifier of the value list this item belongs to.
    pub value_list: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetRadarValueListItemsParams {
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
    pub value: Option<String>,

    pub value_list: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetRadarValueListItemsItemParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostRadarValueListItemsParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The value of the item (whose type must match the type of the parent value list).
    pub value: String,

    /// The identifier of the value list which the created item will be added to.
    pub value_list: String,
}

pub fn get_radar_value_list_items(
    client: &crate::Client,
    params: GetRadarValueListItemsParams,
) -> crate::Response<crate::params::List<crate::generated::RadarValueListItem>> {
    client.get_query("/radar/value_list_items", params)
}

pub fn get_radar_value_list_items_item(
    client: &crate::Client,
    item: String,
    params: GetRadarValueListItemsItemParams,
) -> crate::Response<crate::generated::RadarValueListItem> {
    client.get_query(&format!("/radar/value_list_items/{item}", item = item), params)
}

pub fn post_radar_value_list_items(
    client: &crate::Client,
    params: PostRadarValueListItemsParams,
) -> crate::Response<crate::generated::RadarValueListItem> {
    client.post_form("/radar/value_list_items", params)
}

pub fn delete_radar_value_list_items_item(
    client: &crate::Client,
    item: String,
) -> crate::Response<crate::generated::DeletedRadarValueListItem> {
    client.delete(&format!("/radar/value_list_items/{item}", item = item))
}
