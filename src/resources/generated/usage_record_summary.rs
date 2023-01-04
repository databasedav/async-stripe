#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UsageRecordSummary {
    /// Unique identifier for the object.
    pub id: String,

    /// The invoice in which this usage period has been billed for.
    pub invoice: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub period: crate::generated::Period,

    /// The ID of the subscription item this summary is describing.
    pub subscription_item: String,

    /// The total usage within this usage period.
    pub total_usage: i64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSubscriptionItemsSubscriptionItemUsageRecordSummariesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

pub fn get_subscription_items_subscription_item_usage_record_summaries(
    client: &crate::Client,
    subscription_item: String,
    params: GetSubscriptionItemsSubscriptionItemUsageRecordSummariesParams,
) -> crate::Response<crate::params::List<crate::generated::UsageRecordSummary>> {
    client.get_query(
        &format!(
            "/subscription_items/{subscription_item}/usage_record_summaries",
            subscription_item = subscription_item
        ),
        params,
    )
}
