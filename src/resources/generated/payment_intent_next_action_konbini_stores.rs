#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionKonbiniStores {
    /// FamilyMart instruction details.
    pub familymart: Option<crate::generated::PaymentIntentNextActionKonbiniFamilymart>,

    /// Lawson instruction details.
    pub lawson: Option<crate::generated::PaymentIntentNextActionKonbiniLawson>,

    /// Ministop instruction details.
    pub ministop: Option<crate::generated::PaymentIntentNextActionKonbiniMinistop>,

    /// Seicomart instruction details.
    pub seicomart: Option<crate::generated::PaymentIntentNextActionKonbiniSeicomart>,
}
