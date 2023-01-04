/// The Report Type resource corresponds to a particular type of report, such as
/// the "Activity summary" or "Itemized payouts" reports.
///
/// These objects are identified by an ID belonging to a set of enumerated values.
/// See [API Access to Reports documentation](https://stripe.com/docs/reporting/statements/api) for those Report Type IDs, along with required and optional parameters.  Note that certain report types can only be run based on your live-mode data (not test-mode data), and will error when queried without a [live-mode API key](https://stripe.com/docs/keys#test-live-modes).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ReportingReportType {
    /// Most recent time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_end: crate::params::Timestamp,

    /// Earliest time for which this Report Type is available.
    ///
    /// Measured in seconds since the Unix epoch.
    pub data_available_start: crate::params::Timestamp,

    /// List of column names that are included by default when this Report Type gets run.
    ///
    /// (If the Report Type doesn't support the `columns` parameter, this will be null.).
    pub default_columns: Option<Vec<String>>,

    /// The [ID of the Report Type](https://stripe.com/docs/reporting/statements/api#available-report-types), such as `balance.summary.1`.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Human-readable name of the Report Type.
    pub name: String,

    /// When this Report Type was latest updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::params::Timestamp,

    /// Version of the Report Type.
    ///
    /// Different versions report with the same ID will have the same purpose, but may take different run parameters or have different result schemas.
    pub version: i64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetReportingReportTypesReportTypeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetReportingReportTypesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_reporting_report_types_report_type(
    client: &crate::Client,
    report_type: String,
    params: GetReportingReportTypesReportTypeParams,
) -> crate::Response<crate::generated::ReportingReportType> {
    client.get_query(
        &format!("/reporting/report_types/{report_type}", report_type = report_type),
        params,
    )
}

pub fn get_reporting_report_types(
    client: &crate::Client,
    params: GetReportingReportTypesParams,
) -> crate::Response<crate::params::List<crate::generated::ReportingReportType>> {
    client.get_query("/reporting/report_types", params)
}
