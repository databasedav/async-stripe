#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderIndividual {
    /// The date of birth of this cardholder.
    pub dob: Option<crate::generated::IssuingCardholderIndividualDob>,

    /// The first name of this cardholder.
    pub first_name: String,

    /// The last name of this cardholder.
    pub last_name: String,

    /// Government-issued ID document for this cardholder.
    pub verification: Option<crate::generated::IssuingCardholderVerification>,
}
