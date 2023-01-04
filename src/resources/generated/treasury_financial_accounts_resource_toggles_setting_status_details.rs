/// Additional details on the FinancialAccount Features information.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceTogglesSettingStatusDetails {
    /// Represents the reason why the status is `pending` or `restricted`.
    pub code: TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode,

    /// Represents what the user should do, if anything, to activate the Feature.
    pub resolution: Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution>,

    /// The `platform_restrictions` that are restricting this Feature.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restriction:
        Option<TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    Activating,
    CapabilityNotRequested,
    FinancialAccountClosed,
    RejectedOther,
    RejectedUnsupportedBusiness,
    RequirementsPastDue,
    RequirementsPendingVerification,
    RestrictedByPlatform,
    RestrictedOther,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Activating => "activating",
            Self::CapabilityNotRequested => "capability_not_requested",
            Self::FinancialAccountClosed => "financial_account_closed",
            Self::RejectedOther => "rejected_other",
            Self::RejectedUnsupportedBusiness => "rejected_unsupported_business",
            Self::RequirementsPastDue => "requirements_past_due",
            Self::RequirementsPendingVerification => "requirements_pending_verification",
            Self::RestrictedByPlatform => "restricted_by_platform",
            Self::RestrictedOther => "restricted_other",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsCode {
    fn default() -> Self {
        Self::Activating
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    ContactStripe,
    ProvideInformation,
    RemoveRestriction,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ContactStripe => "contact_stripe",
            Self::ProvideInformation => "provide_information",
            Self::RemoveRestriction => "remove_restriction",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsResolution {
    fn default() -> Self {
        Self::ContactStripe
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    InboundFlows,
    OutboundFlows,
}

impl TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InboundFlows => "inbound_flows",
            Self::OutboundFlows => "outbound_flows",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceTogglesSettingStatusDetailsRestriction {
    fn default() -> Self {
        Self::InboundFlows
    }
}
