#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingTransactionLodgingData {
    /// The time of checking into the lodging.
    pub check_in_at: Option<i64>,

    /// The number of nights stayed at the lodging.
    pub nights: Option<i64>,
}
