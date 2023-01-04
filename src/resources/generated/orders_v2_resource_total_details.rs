#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceTotalDetails {
    /// This is the sum of all the discounts.
    pub amount_discount: i64,

    /// This is the sum of all the shipping amounts.
    pub amount_shipping: Option<i64>,

    /// This is the sum of all the tax amounts.
    pub amount_tax: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub breakdown: Option<crate::generated::OrdersV2ResourceTotalDetailsApiResourceBreakdown>,
}
