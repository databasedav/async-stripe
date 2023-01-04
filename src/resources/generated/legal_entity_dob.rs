#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct LegalEntityDob {
    /// The day of birth, between 1 and 31.
    pub day: Option<i64>,

    /// The month of birth, between 1 and 12.
    pub month: Option<i64>,

    /// The four-digit year of birth.
    pub year: Option<i64>,
}
