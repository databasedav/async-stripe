#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CountrySpecVerificationFieldDetails {
    /// Additional fields which are only required for some users.
    pub additional: Vec<String>,

    /// Fields which every account must eventually provide.
    pub minimum: Vec<String>,
}
