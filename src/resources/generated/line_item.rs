#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct LineItem {
    /// The amount, in %s.
    pub amount: i64,

    /// The integer amount in %s representing the amount for this line item, excluding all tax and discounts.
    pub amount_excluding_tax: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The amount of discount calculated per discount for this line item.
    pub discount_amounts: Option<Vec<crate::generated::DiscountsResourceDiscountAmount>>,

    /// If true, discounts will apply to this line item.
    ///
    /// Always false for prorations.
    pub discountable: bool,

    /// The discounts applied to the invoice line item.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<Vec<crate::generated::Discount>>>,

    /// Unique identifier for the object.
    pub id: String,

    /// The ID of the [invoice item](https://stripe.com/docs/api/invoiceitems) associated with this line item if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_item: Option<String>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Note that for line items with `type=subscription` this will reflect the metadata of the subscription that caused the line item to be created.
    pub metadata: crate::params::Metadata,

    pub period: crate::generated::InvoiceLineItemPeriod,

    /// The plan of the subscription, if the line item is a subscription or a proration.
    pub plan: Option<crate::generated::Plan>,

    /// The price of the line item.
    pub price: Option<crate::generated::Price>,

    /// Whether this is a proration.
    pub proration: bool,

    /// Additional details for proration line items.
    pub proration_details: Option<crate::generated::InvoicesLineItemsProrationDetails>,

    /// The quantity of the subscription, if the line item is a subscription or a proration.
    pub quantity: Option<u64>,

    /// The subscription that the invoice item pertains to, if any.
    pub subscription: Option<String>,

    /// The subscription item that generated this invoice item.
    ///
    /// Left empty if the line item is not an explicit result of a subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The amount of tax calculated per tax rate for this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_amounts: Option<Vec<crate::generated::InvoiceTaxAmount>>,

    /// The tax rates which apply to the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<crate::generated::TaxRate>>,

    /// A string identifying the type of the source of this line item, either an `invoiceitem` or a `subscription`.
    #[serde(rename = "type")]
    pub type_: LineItemType,

    /// The amount in %s representing the unit amount for this line item, excluding all tax and discounts.
    pub unit_amount_excluding_tax: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesInvoiceLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum LineItemType {
    Invoiceitem,
    Subscription,
}

impl LineItemType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Invoiceitem => "invoiceitem",
            Self::Subscription => "subscription",
        }
    }
}

impl AsRef<str> for LineItemType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for LineItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for LineItemType {
    fn default() -> Self {
        Self::Invoiceitem
    }
}
pub fn get_invoices_invoice_lines(
    client: &crate::Client,
    invoice: String,
    params: GetInvoicesInvoiceLinesParams,
) -> crate::Response<crate::params::List<crate::generated::LineItem>> {
    client.get_query(&format!("/invoices/{invoice}/lines", invoice = invoice), params)
}
