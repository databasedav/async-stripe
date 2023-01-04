#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardWallets {
    pub apple_pay: crate::generated::IssuingCardApplePay,

    pub google_pay: crate::generated::IssuingCardGooglePay,

    /// Unique identifier for a card used with digital wallets.
    pub primary_account_identifier: Option<String>,
}
