/// Toggle settings for enabling/disabling an ACH specific feature.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceAchToggleSettings {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,

    /// Whether the Feature is operational.
    pub status: TreasuryFinancialAccountsResourceAchToggleSettingsStatus,

    /// Additional details; includes at least one entry when the status is not `active`.
    pub status_details:
        Vec<crate::generated::TreasuryFinancialAccountsResourceTogglesSettingStatusDetails>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    Active,
    Pending,
    Restricted,
}

impl TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Pending => "pending",
            Self::Restricted => "restricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceAchToggleSettingsStatus {
    fn default() -> Self {
        Self::Active
    }
}
