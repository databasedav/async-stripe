/// Usage records allow you to report customer usage and metrics to Stripe for
/// metered billing of subscription prices.
///
/// Related guide: [Metered Billing](https://stripe.com/docs/billing/subscriptions/metered-billing).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,

    /// The timestamp when this usage occurred.
    pub timestamp: crate::params::Timestamp,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionItemsSubscriptionItemUsageRecordsParams {
    /// Valid values are `increment` (default) or `set`.
    ///
    /// When using `increment` the specified `quantity` will be added to the usage at the specified timestamp.
    /// The `set` action will overwrite the usage quantity at that timestamp.
    /// If the subscription has [billing thresholds](https://stripe.com/docs/api/subscriptions/object#subscription_object-billing_thresholds), `increment` is the only allowed value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub action: Option<PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The usage quantity for the specified timestamp.
    pub quantity: u64,

    /// The timestamp for the usage event.
    ///
    /// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
    /// When passing `"now"`, Stripe records usage for the current time.
    /// Default is `"now"` if a value is not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timestamp: Option<PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction {
    Increment,
    Set,
}

impl PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Increment => "increment",
            Self::Set => "set",
        }
    }
}

impl AsRef<str> for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsAction {
    fn default() -> Self {
        Self::Increment
    }
}

/// The timestamp for the usage event.
///
/// This timestamp must be within the current billing period of the subscription of the provided `subscription_item`, and must not be in the future.
/// When passing `"now"`, Stripe records usage for the current time.
/// Default is `"now"` if a value is not provided.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestamp {
    Now(PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow),
    I64(i64),
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow {
    Now,
}

impl PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionItemsSubscriptionItemUsageRecordsParamsTimestampNow {
    fn default() -> Self {
        Self::Now
    }
}
pub fn post_subscription_items_subscription_item_usage_records(
    client: &crate::Client,
    subscription_item: String,
    params: PostSubscriptionItemsSubscriptionItemUsageRecordsParams,
) -> crate::Response<crate::generated::UsageRecord> {
    client.post_form(
        &format!(
            "/subscription_items/{subscription_item}/usage_records",
            subscription_item = subscription_item
        ),
        params,
    )
}
