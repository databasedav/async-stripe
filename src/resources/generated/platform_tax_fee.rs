#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PlatformTaxFee {
    /// The Connected account that incurred this charge.
    pub account: String,

    /// Unique identifier for the object.
    pub id: String,

    /// The payment object that caused this tax to be inflicted.
    pub source_transaction: String,

    /// The type of tax (VAT).
    #[serde(rename = "type")]
    pub type_: String,
}
