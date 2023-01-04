/// Toggle settings for enabling/disabling a feature.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<crate::generated::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}
