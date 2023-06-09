#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionShippingCost {
    /// Total shipping cost before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied due to shipping costs.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total shipping cost after discounts and taxes are applied.
    pub amount_total: i64,

    /// The ID of the ShippingRate for this order.
    pub shipping_rate: Option<Vec<crate::generated::ShippingRate>>,

    /// The taxes applied to the shipping rate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<crate::generated::LineItemsTaxAmount>>,
}