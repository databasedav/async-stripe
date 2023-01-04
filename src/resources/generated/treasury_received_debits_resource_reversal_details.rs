#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedDebitsResourceReversalDetails {
    /// Time before which a ReceivedDebit can be reversed.
    pub deadline: Option<crate::params::Timestamp>,

    /// Set if a ReceivedDebit can't be reversed.
    pub restricted_reason: Option<TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    AlreadyReversed,
    DeadlinePassed,
    NetworkRestricted,
    Other,
    SourceFlowRestricted,
}

impl TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
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

impl AsRef<str> for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedDebitsResourceReversalDetailsRestrictedReason {
    fn default() -> Self {
        Self::AlreadyReversed
    }
}
