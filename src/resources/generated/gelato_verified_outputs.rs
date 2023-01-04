#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GelatoVerifiedOutputs {
    /// The user's verified address.
    pub address: Option<crate::generated::Address>,

    /// The user’s verified date of birth.
    pub dob: Option<crate::generated::GelatoDataVerifiedOutputsDate>,

    /// The user's verified first name.
    pub first_name: Option<String>,

    /// The user's verified id number.
    pub id_number: Option<String>,

    /// The user's verified id number type.
    pub id_number_type: Option<GelatoVerifiedOutputsIdNumberType>,

    /// The user's verified last name.
    pub last_name: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GelatoVerifiedOutputsIdNumberType {
    BrCpf,
    SgNric,
    UsSsn,
}

impl GelatoVerifiedOutputsIdNumberType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BrCpf => "br_cpf",
            Self::SgNric => "sg_nric",
            Self::UsSsn => "us_ssn",
        }
    }
}

impl AsRef<str> for GelatoVerifiedOutputsIdNumberType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GelatoVerifiedOutputsIdNumberType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GelatoVerifiedOutputsIdNumberType {
    fn default() -> Self {
        Self::BrCpf
    }
}
