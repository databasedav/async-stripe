#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryInboundTransfersResourceInboundTransferResourceStatusTransitions {
    /// Timestamp describing when an InboundTransfer changed status to `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when an InboundTransfer changed status to `failed`.
    pub failed_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when an InboundTransfer changed status to `succeeded`.
    pub succeeded_at: Option<crate::params::Timestamp>,
}
