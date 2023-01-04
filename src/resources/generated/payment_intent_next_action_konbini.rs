#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionKonbini {
    /// The timestamp at which the pending Konbini payment expires.
    pub expires_at: crate::resources::Scheduled,

    /// The URL for the Konbini payment instructions page, which allows customers to view and print a Konbini voucher.
    pub hosted_voucher_url: Option<String>,

    pub stores: crate::generated::PaymentIntentNextActionKonbiniStores,
}
