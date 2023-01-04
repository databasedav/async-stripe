/// A line item.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Item {
    /// Total discount amount applied.
    ///
    /// If no discounts were applied, defaults to 0.
    pub amount_discount: i64,

    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total tax amount applied.
    ///
    /// If no tax was applied, defaults to 0.
    pub amount_tax: i64,

    /// Total after discounts and taxes.
    pub amount_total: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Defaults to product name.
    pub description: String,

    /// The discounts applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<crate::generated::LineItemsDiscountAmount>>,

    /// Unique identifier for the object.
    pub id: String,

    /// The price used to generate the line item.
    pub price: Option<crate::generated::Price>,

    /// The ID of the product for this line item.
    ///
    /// This will always be the same as `price.product`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<Vec<crate::generated::Product>>,

    /// The quantity of products being purchased.
    pub quantity: Option<u64>,

    /// The taxes applied to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub taxes: Option<Vec<crate::generated::LineItemsTaxAmount>>,
}
