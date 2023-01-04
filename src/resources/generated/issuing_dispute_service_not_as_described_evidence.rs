#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeServiceNotAsDescribedEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Date when order was canceled.
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Reason for canceling the order.
    pub cancellation_reason: Option<String>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,

    /// Date when the product was received.
    pub received_at: Option<crate::params::Timestamp>,
}
