#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedCreditsResourceReversalDetails {
    /// Time before which a ReceivedCredit can be reversed.
    pub deadline: Option<crate::params::Timestamp>,

    /// Set if a ReceivedCredit cannot be reversed.
    pub restricted_reason: Option<TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlreadyReversed => "already_reversed",
            Self::DeadlinePassed => "deadline_passed",
            Self::NetworkRestricted => "network_restricted",
            Self::Other => "other",
            Self::SourceFlowRestricted => "source_flow_restricted",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedCreditsResourceReversalDetailsRestrictedReason {
    fn default() -> Self {
        Self::AlreadyReversed
    }
}
