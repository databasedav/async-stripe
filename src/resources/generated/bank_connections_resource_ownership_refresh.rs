#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BankConnectionsResourceOwnershipRefresh {
    /// The time at which the last refresh attempt was initiated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub last_attempted_at: crate::params::Timestamp,

    /// The status of the last refresh attempt.
    pub status: BankConnectionsResourceOwnershipRefreshStatus,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceOwnershipRefreshStatus {
    Failed,
    Pending,
    Succeeded,
}

impl BankConnectionsResourceOwnershipRefreshStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Pending => "pending",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceOwnershipRefreshStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceOwnershipRefreshStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for BankConnectionsResourceOwnershipRefreshStatus {
    fn default() -> Self {
        Self::Failed
    }
}
