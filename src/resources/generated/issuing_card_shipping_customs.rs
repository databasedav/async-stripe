#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardShippingCustoms {
    /// A registration number used for customs in Europe.
    ///
    /// See https://www.gov.uk/eori and https://ec.europa.eu/taxation_customs/business/customs-procedures-import-and-export/customs-procedures/economic-operators-registration-and-identification-number-eori_en.
    pub eori_number: Option<String>,
}
