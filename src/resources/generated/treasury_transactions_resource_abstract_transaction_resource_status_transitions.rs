#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions {
    /// Timestamp describing when the Transaction changed status to `posted`.
    pub posted_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when the Transaction changed status to `void`.
    pub void_at: Option<crate::params::Timestamp>,
}
