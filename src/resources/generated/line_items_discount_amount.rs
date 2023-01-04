#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LineItemsDiscountAmount {
    /// The amount discounted.
    pub amount: i64,

    pub discount: crate::generated::Discount,
}
