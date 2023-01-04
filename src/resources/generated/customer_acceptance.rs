#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerAcceptance {
    /// The time at which the customer accepted the Mandate.
    pub accepted_at: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<crate::generated::OfflineAcceptance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<crate::generated::OnlineAcceptance>,

    /// The type of customer acceptance information included with the Mandate.
    ///
    /// One of `online` or `offline`.
    #[serde(rename = "type")]
    pub type_: CustomerAcceptanceType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerAcceptanceType {
    Offline,
    Online,
}

impl CustomerAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for CustomerAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerAcceptanceType {
    fn default() -> Self {
        Self::Offline
    }
}
