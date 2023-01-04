/// If you have [scheduled a Sigma query](https://stripe.com/docs/sigma/scheduled-queries), you'll
/// receive a `sigma.scheduled_query_run.created` webhook each time the query
/// runs.
///
/// The webhook contains a `ScheduledQueryRun` object, which you can use to retrieve the query results.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ScheduledQueryRun {
    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// When the query was run, Sigma contained a snapshot of your Stripe data at this time.
    pub data_load_time: crate::params::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<crate::generated::SigmaScheduledQueryRunError>,

    /// The file object representing the results of the query.
    pub file: Option<crate::generated::File>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Time at which the result expires and is no longer available for download.
    pub result_available_until: crate::params::Timestamp,

    /// SQL for the query.
    pub sql: String,

    /// The query's execution status, which will be `completed` for successful runs, and `canceled`, `failed`, or `timed_out` otherwise.
    pub status: String,

    /// Title of the query.
    pub title: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSigmaScheduledQueryRunsParams {
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
pub struct GetSigmaScheduledQueryRunsScheduledQueryRunParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_sigma_scheduled_query_runs(
    client: &crate::Client,
    params: GetSigmaScheduledQueryRunsParams,
) -> crate::Response<crate::params::List<crate::generated::ScheduledQueryRun>> {
    client.get_query("/sigma/scheduled_query_runs", params)
}

pub fn get_sigma_scheduled_query_runs_scheduled_query_run(
    client: &crate::Client,
    scheduled_query_run: String,
    params: GetSigmaScheduledQueryRunsScheduledQueryRunParams,
) -> crate::Response<crate::generated::ScheduledQueryRun> {
    client.get_query(
        &format!(
            "/sigma/scheduled_query_runs/{scheduled_query_run}",
            scheduled_query_run = scheduled_query_run
        ),
        params,
    )
}
