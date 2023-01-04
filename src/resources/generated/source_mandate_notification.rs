/// Source mandate notifications should be created when a notification related to
/// a source mandate must be sent to the payer.
///
/// They will trigger a webhook or deliver an email to the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SourceMandateNotification {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::SourceMandateNotificationAcssDebitData>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount associated with the mandate notification.
    ///
    /// The amount is expressed in the currency of the underlying source.
    /// Required if the notification type is `debit_initiated`.
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::SourceMandateNotificationBacsDebitData>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The reason of the mandate notification.
    ///
    /// Valid reasons are `mandate_confirmed` or `debit_initiated`.
    pub reason: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::SourceMandateNotificationSepaDebitData>,

    pub source: crate::generated::Source,

    /// The status of the mandate notification.
    ///
    /// Valid statuses are `pending` or `submitted`.
    pub status: String,

    /// The type of source this mandate notification is attached to.
    ///
    /// Should be the source type identifier code for the payment method, such as `three_d_secure`.
    #[serde(rename = "type")]
    pub type_: String,
}
