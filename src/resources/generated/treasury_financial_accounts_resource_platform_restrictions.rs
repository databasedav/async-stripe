/// Restrictions that a Connect Platform has placed on this FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourcePlatformRestrictions {
    /// Restricts all inbound money movement.
    pub inbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows>,

    /// Restricts all outbound money movement.
    pub outbound_flows: Option<TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourcePlatformRestrictionsInboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourcePlatformRestrictionsOutboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}
