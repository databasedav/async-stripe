#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionScheduleCurrentPhase {
    /// The end of this phase of the subscription schedule.
    pub end_date: crate::params::Timestamp,

    /// The start of this phase of the subscription schedule.
    pub start_date: crate::params::Timestamp,
}
