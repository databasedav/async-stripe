#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CreditNoteTaxAmount {
    /// The amount, in %s, of the tax.
    pub amount: i64,

    /// Whether this tax amount is inclusive or exclusive.
    pub inclusive: bool,

    /// The tax rate that was applied to get this tax amount.
    pub tax_rate: Vec<crate::generated::TaxRate>,
}
