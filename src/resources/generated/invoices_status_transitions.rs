#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct InvoicesStatusTransitions {
    /// The time that the invoice draft was finalized.
    pub finalized_at: Option<crate::params::Timestamp>,

    /// The time that the invoice was marked uncollectible.
    pub marked_uncollectible_at: Option<crate::params::Timestamp>,

    /// The time that the invoice was paid.
    pub paid_at: Option<crate::params::Timestamp>,

    /// The time that the invoice was voided.
    pub voided_at: Option<crate::params::Timestamp>,
}
