#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct DeletedDiscount {
    /// The Checkout session that this coupon is applied to, if it is applied to a particular session in payment mode.
    ///
    /// Will not be present for subscription mode.
    pub checkout_session: Option<String>,

    pub coupon: crate::generated::Coupon,

    /// The ID of the customer associated with this discount.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// Always true for a deleted object.
    pub deleted: bool,

    /// The ID of the discount object.
    ///
    /// Discounts cannot be fetched by ID.
    /// Use `expand[]=discounts` in API calls to expand discount IDs in an array.
    pub id: String,

    /// The invoice that the discount's coupon was applied to, if it was applied directly to a particular invoice.
    pub invoice: Option<String>,

    /// The invoice item `id` (or invoice line item `id` for invoice line items of type='subscription') that the discount's coupon was applied to, if it was applied directly to a particular invoice item or invoice line item.
    pub invoice_item: Option<String>,

    /// The promotion code applied to create this discount.
    pub promotion_code: Option<Vec<crate::generated::PromotionCode>>,

    /// Date that the coupon was applied.
    pub start: crate::params::Timestamp,

    /// The subscription that this coupon is applied to, if it is applied to a particular subscription.
    pub subscription: Option<String>,
}