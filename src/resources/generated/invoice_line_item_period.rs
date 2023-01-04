#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoiceLineItemPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::params::Timestamp,

    /// The start of the period.
    pub start: crate::params::Timestamp,
}
