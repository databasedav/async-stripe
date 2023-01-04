#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardholderAddress {
    pub address: crate::generated::Address,
}
