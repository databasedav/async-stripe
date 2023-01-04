#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedDebitsResourceDebitReversalLinkedFlows {
    /// Set if there is an Issuing dispute associated with the DebitReversal.
    pub issuing_dispute: Option<String>,
}
