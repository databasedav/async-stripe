#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceClosedStatusDetails {
    /// The array that contains reasons for a FinancialAccount closure.
    pub reasons: Vec<TreasuryFinancialAccountsResourceClosedStatusDetailsReasons>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    AccountRejected,
    ClosedByPlatform,
    Other,
}

impl TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AccountRejected => "account_rejected",
            Self::ClosedByPlatform => "closed_by_platform",
            Self::Other => "other",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceClosedStatusDetailsReasons {
    fn default() -> Self {
        Self::AccountRejected
    }
}
