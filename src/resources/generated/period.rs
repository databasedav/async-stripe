#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct Period {
    /// The end date of this usage period.
    ///
    /// All usage up to and including this point in time is included.
    pub end: Option<crate::params::Timestamp>,

    /// The start date of this usage period.
    ///
    /// All usage after this point in time is included.
    pub start: Option<crate::params::Timestamp>,
}
