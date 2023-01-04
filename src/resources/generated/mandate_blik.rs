#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct MandateBlik {
    /// Date at which the mandate expires.
    pub expires_after: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<crate::generated::MandateOptionsOffSessionDetailsBlik>,

    /// Type of the mandate.
    #[serde(rename = "type")]
    pub type_: Option<MandateBlikType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateBlikType {
    OffSession,
    OnSession,
}

impl MandateBlikType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for MandateBlikType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateBlikType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for MandateBlikType {
    fn default() -> Self {
        Self::OffSession
    }
}
