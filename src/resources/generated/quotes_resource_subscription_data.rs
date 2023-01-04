#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct QuotesResourceSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    pub description: Option<String>,

    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// This date is ignored if it is in the past when the quote is accepted.
    /// Measured in seconds since the Unix epoch.
    pub effective_date: Option<crate::params::Timestamp>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    pub trial_period_days: Option<u32>,
}
