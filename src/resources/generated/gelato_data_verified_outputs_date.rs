/// Point in Time.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoDataVerifiedOutputsDate {
    /// Numerical day between 1 and 31.
    pub day: Option<i64>,

    /// Numerical month between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year.
    pub year: Option<i64>,
}
