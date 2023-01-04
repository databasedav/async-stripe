#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LineItemsTaxAmount {
    /// Amount of tax applied for this rate.
    pub amount: i64,

    pub rate: crate::generated::TaxRate,
}
