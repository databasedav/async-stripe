#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DiscountsResourceDiscountAmount {
    /// The amount, in %s, of the discount.
    pub amount: i64,

    /// The discount that was applied to get this discount amount.
    pub discount: Vec<crate::generated::Discount>,
}
