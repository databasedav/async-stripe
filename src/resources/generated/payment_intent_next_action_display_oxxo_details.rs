#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionDisplayOxxoDetails {
    /// The timestamp after which the OXXO voucher expires.
    pub expires_after: Option<crate::params::Timestamp>,

    /// The URL for the hosted OXXO voucher page, which allows customers to view and print an OXXO voucher.
    pub hosted_voucher_url: Option<String>,

    /// OXXO reference number.
    pub number: Option<String>,
}
