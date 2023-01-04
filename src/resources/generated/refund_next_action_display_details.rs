#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RefundNextActionDisplayDetails {
    pub email_sent: crate::generated::EmailSent,

    /// The expiry timestamp.
    pub expires_at: crate::resources::Scheduled,
}
