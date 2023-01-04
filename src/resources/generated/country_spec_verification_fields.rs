#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CountrySpecVerificationFields {
    pub company: crate::generated::CountrySpecVerificationFieldDetails,

    pub individual: crate::generated::CountrySpecVerificationFieldDetails,
}
