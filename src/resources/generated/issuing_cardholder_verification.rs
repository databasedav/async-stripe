#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderVerification {
    /// An identifying document, either a passport or local ID card.
    pub document: Option<crate::generated::IssuingCardholderIdDocument>,
}
