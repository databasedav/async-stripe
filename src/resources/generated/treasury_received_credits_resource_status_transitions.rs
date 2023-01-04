#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedCreditsResourceStatusTransitions {
    /// Timestamp describing when the CreditReversal changed status to `posted`.
    pub posted_at: Option<crate::params::Timestamp>,
}
