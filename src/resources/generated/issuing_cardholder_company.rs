#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderCompany {
    /// Whether the company's business ID number was provided.
    pub tax_id_provided: bool,
}
