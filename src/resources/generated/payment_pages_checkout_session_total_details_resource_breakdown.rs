#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionTotalDetailsResourceBreakdown {
    /// The aggregated discounts.
    pub discounts: Vec<crate::generated::LineItemsDiscountAmount>,

    /// The aggregated tax amounts by rate.
    pub taxes: Vec<crate::generated::LineItemsTaxAmount>,
}