#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct QuotesResourceComputed {
    /// The definitive totals and line items the customer will be charged on a recurring basis.
    ///
    /// Takes into account the line items with recurring prices and discounts with `duration=forever` coupons only.
    /// Defaults to `null` if no inputted line items with recurring prices.
    pub recurring: Option<crate::generated::QuotesResourceRecurring>,

    pub upfront: crate::generated::QuotesResourceUpfront,
}
