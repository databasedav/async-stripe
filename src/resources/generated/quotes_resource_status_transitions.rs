#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct QuotesResourceStatusTransitions {
    /// The time that the quote was accepted.
    ///
    /// Measured in seconds since Unix epoch.
    pub accepted_at: Option<crate::params::Timestamp>,

    /// The time that the quote was canceled.
    ///
    /// Measured in seconds since Unix epoch.
    pub canceled_at: Option<crate::params::Timestamp>,

    /// The time that the quote was finalized.
    ///
    /// Measured in seconds since Unix epoch.
    pub finalized_at: Option<crate::params::Timestamp>,
}
