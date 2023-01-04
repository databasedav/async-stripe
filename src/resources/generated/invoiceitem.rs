/// Sometimes you want to add a charge or credit to a customer, but actually
/// charge or credit the customer's card only at the end of a regular billing
/// cycle.
///
/// This is useful for combining several charges (to minimize per-transaction fees), or for having Stripe tabulate your usage-based billing totals.  Related guide: [Subscription Invoices](https://stripe.com/docs/billing/invoices/subscription#adding-upcoming-invoice-items).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Invoiceitem {
    /// Amount (in the `currency` specified) of the invoice item.
    ///
    /// This should always be equal to `unit_amount * quantity`.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: Vec<crate::generated::Customer>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub date: crate::params::Timestamp,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// If true, discounts will apply to this invoice item.
    ///
    /// Always false for prorations.
    pub discountable: bool,

    /// The discounts which apply to the invoice item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<Vec<crate::generated::Discount>>>,

    /// Unique identifier for the object.
    pub id: String,

    /// The ID of the invoice this invoice item belongs to.
    pub invoice: Option<Vec<crate::generated::Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    pub period: crate::generated::InvoiceLineItemPeriod,

    /// If the invoice item is a proration, the plan of the subscription that the proration was computed for.
    pub plan: Option<crate::generated::Plan>,

    /// The price of the invoice item.
    pub price: Option<crate::generated::Price>,

    /// Whether the invoice item was created automatically as a proration adjustment when the customer switched plans.
    pub proration: bool,

    /// Quantity of units for the invoice item.
    ///
    /// If the invoice item is a proration, the quantity of the subscription that the proration was computed for.
    pub quantity: u64,

    /// The subscription that this invoice item has been created for, if any.
    pub subscription: Option<Vec<crate::generated::Subscription>>,

    /// The subscription item that this invoice item has been created for, if any.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_item: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    pub tax_rates: Option<Vec<crate::generated::TaxRate>>,

    /// ID of the test clock this invoice item belongs to.
    pub test_clock: Option<Vec<crate::generated::TestHelpersTestClock>>,

    /// Unit amount (in the `currency` specified) of the invoice item.
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but contains a decimal value with at most 12 decimal places.
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoiceitemsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsParams {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// Passing in a negative `amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The ID of the customer who will be billed when this invoice item is billed.
    pub customer: String,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons to redeem into discounts for the invoice item or invoice line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostInvoiceitemsParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The ID of an existing invoice to add this invoice item to.
    ///
    /// When left blank, the invoice item will be added to the next upcoming scheduled invoice.
    /// This is useful when adding invoice items in response to an invoice.created webhook.
    /// You can only add invoice items to draft invoices and there is a maximum of 250 items per invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PostInvoiceitemsParamsPeriod>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostInvoiceitemsParamsPriceData>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The ID of a subscription to add this invoice item to.
    ///
    /// When left blank, the invoice item will be be added to the next upcoming scheduled invoice.
    /// When set, scheduled invoices for subscriptions other than the specified subscription will ignore the invoice item.
    /// Use this when you want to express that an invoice item has been accrued within the context of a particular subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostInvoiceitemsParamsTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount.
    /// Passing in a negative `unit_amount` will reduce the `amount_due` on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoiceitemsInvoiceitemParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsInvoiceitemParams {
    /// The integer amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// If you want to apply a credit to the customer's account, pass a negative amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Controls whether discounts apply to this invoice item.
    ///
    /// Defaults to false for prorations or negative invoice items, and true for all other invoice items.
    /// Cannot be set to true for prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons & existing discounts which apply to the invoice item or invoice line item.
    ///
    /// Item discounts are applied before invoice discounts.
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostInvoiceitemsInvoiceitemParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The period associated with this invoice item.
    ///
    /// When set to different values, the period will be rendered on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub period: Option<PostInvoiceitemsInvoiceitemParamsPeriod>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostInvoiceitemsInvoiceitemParamsPriceData>,

    /// Non-negative integer.
    ///
    /// The quantity of units for the invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostInvoiceitemsInvoiceitemParamsTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates which apply to the invoice item.
    ///
    /// When set, the `default_tax_rates` on the invoice do not apply to this invoice item.
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// The integer unit amount in cents (or local equivalent) of the charge to be applied to the upcoming invoice.
    ///
    /// This unit_amount will be multiplied by the quantity to get the full amount.
    /// If you want to apply a credit to the customer's account, pass a negative unit_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsParamsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::params::Timestamp,

    /// The start of the period.
    pub start: crate::params::Timestamp,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsParamsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostInvoiceitemsParamsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoiceitemsParamsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostInvoiceitemsParamsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostInvoiceitemsParamsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoiceitemsParamsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoiceitemsParamsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsInvoiceitemParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// The period associated with this invoice item.
///
/// When set to different values, the period will be rendered on the invoice.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsInvoiceitemParamsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::params::Timestamp,

    /// The start of the period.
    pub start: crate::params::Timestamp,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoiceitemsInvoiceitemParamsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoiceitemsInvoiceitemParamsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostInvoiceitemsInvoiceitemParamsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostInvoiceitemsInvoiceitemParamsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoiceitemsInvoiceitemParamsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoiceitemsInvoiceitemParamsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoiceitemsParamsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostInvoiceitemsParamsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostInvoiceitemsParamsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoiceitemsParamsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoiceitemsParamsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoiceitemsInvoiceitemParamsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
pub fn get_invoiceitems(
    client: &crate::Client,
    params: GetInvoiceitemsParams,
) -> crate::Response<crate::params::List<crate::generated::Invoiceitem>> {
    client.get_query("/invoiceitems", params)
}

pub fn post_invoiceitems(
    client: &crate::Client,
    params: PostInvoiceitemsParams,
) -> crate::Response<crate::generated::Invoiceitem> {
    client.post_form("/invoiceitems", params)
}

pub fn get_invoiceitems_invoiceitem(
    client: &crate::Client,
    invoiceitem: String,
    params: GetInvoiceitemsInvoiceitemParams,
) -> crate::Response<crate::generated::Invoiceitem> {
    client.get_query(&format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem), params)
}

pub fn post_invoiceitems_invoiceitem(
    client: &crate::Client,
    invoiceitem: String,
    params: PostInvoiceitemsInvoiceitemParams,
) -> crate::Response<crate::generated::Invoiceitem> {
    client.post_form(&format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem), params)
}

pub fn delete_invoiceitems_invoiceitem(
    client: &crate::Client,
    invoiceitem: String,
) -> crate::Response<crate::generated::DeletedInvoiceitem> {
    client.delete(&format!("/invoiceitems/{invoiceitem}", invoiceitem = invoiceitem))
}
