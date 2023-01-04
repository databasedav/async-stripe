#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct MandateSingleUse {
    /// On a single use mandate, the amount of the payment.
    pub amount: i64,

    /// On a single use mandate, the currency of the payment.
    pub currency: crate::currency::Currency,
}
