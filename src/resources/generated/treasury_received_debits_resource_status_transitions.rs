#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedDebitsResourceStatusTransitions {
    /// Timestamp describing when the DebitReversal changed status to `completed`.
    pub completed_at: Option<crate::params::Timestamp>,
}
