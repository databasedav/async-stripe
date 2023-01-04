#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PromotionCodeCurrencyOption {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    pub minimum_amount: i64,
}
