#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CouponAppliesTo {
    /// A list of product IDs this coupon applies to.
    pub products: Vec<String>,
}
