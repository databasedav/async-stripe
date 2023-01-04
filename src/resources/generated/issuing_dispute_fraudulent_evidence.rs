#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct IssuingDisputeFraudulentEvidence {
    /// (ID of a [file upload](https://stripe.com/docs/guides/file-upload)) Additional documentation supporting the dispute.
    pub additional_documentation: Option<Vec<crate::generated::File>>,

    /// Explanation of why the cardholder is disputing this transaction.
    pub explanation: Option<String>,
}
