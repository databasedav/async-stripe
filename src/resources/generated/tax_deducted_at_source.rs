#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TaxDeductedAtSource {
    /// Unique identifier for the object.
    pub id: String,

    /// The end of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_end: crate::params::Timestamp,

    /// The start of the invoicing period.
    ///
    /// This TDS applies to Stripe fees collected during this invoicing period.
    pub period_start: crate::params::Timestamp,

    /// The TAN that was supplied to Stripe when TDS was assessed.
    pub tax_deduction_account_number: String,
}
