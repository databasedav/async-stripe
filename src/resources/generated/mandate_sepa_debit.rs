#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MandateSepaDebit {
    /// The unique reference of the mandate.
    pub reference: String,

    /// The URL of the mandate.
    ///
    /// This URL generally contains sensitive information about the customer and should be shared with them exclusively.
    pub url: String,
}