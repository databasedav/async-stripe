#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TreasuryOutboundPaymentsResourceOutboundPaymentResourceStatusTransitions {
    /// Timestamp describing when an OutboundPayment changed status to `canceled`.
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `failed`.
    pub failed_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `posted`.
    pub posted_at: Option<crate::params::Timestamp>,

    /// Timestamp describing when an OutboundPayment changed status to `returned`.
    pub returned_at: Option<crate::params::Timestamp>,
}
