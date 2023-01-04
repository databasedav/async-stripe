#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct EmailSent {
    /// The timestamp when the email was sent.
    pub email_sent_at: crate::params::Timestamp,

    /// The recipient's email address.
    pub email_sent_to: String,
}
