/// An Order describes a purchase being made by a customer, including the
/// products & quantities being purchased, the order status, the payment information,
/// and the billing/shipping details.
///
/// Related guide: [Orders overview](https://stripe.com/docs/orders).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Order {
    /// Order cost before any discounts or taxes are applied.
    ///
    /// A positive integer representing the subtotal of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    pub amount_subtotal: i64,

    /// Total order cost after discounts and taxes are applied.
    ///
    /// A positive integer representing the cost of the order in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to charge $1.00 or 100 to charge ¥100, a zero-decimal currency).
    /// To submit an order, the total must be either 0 or at least $0.50 USD or [equivalent in charge currency](https://stripe.com/docs/currencies#minimum-and-maximum-charge-amounts).
    pub amount_total: i64,

    /// ID of the Connect application that created the Order, if any.
    pub application: Option<Vec<crate::generated::Application>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::generated::OrdersV2ResourceAutomaticTax>,

    /// Customer billing details associated with the order.
    pub billing_details: Option<crate::generated::OrdersV2ResourceBillingDetails>,

    /// The client secret of this Order.
    ///
    /// Used for client-side retrieval using a publishable key.
    /// The client secret can be used to complete a payment for an Order from your frontend.
    /// It should not be stored, logged, embedded in URLs, or exposed to anyone other than the customer.
    /// Make sure that you have TLS enabled on any page that includes the client secret.
    /// Refer to our docs for [creating and processing an order](https://stripe.com/docs/orders-beta/create-and-process) to learn about how client_secret should be handled.
    pub client_secret: Option<String>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The customer which this orders belongs to.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The discounts applied to the order.
    ///
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<Vec<crate::generated::Discount>>>,

    /// Unique identifier for the object.
    pub id: String,

    /// A recent IP address of the purchaser used for tax reporting and tax location inference.
    pub ip_address: Option<String>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    /// There is a maximum of 100 line items.
    #[serde(default)]
    pub line_items: crate::params::List<crate::generated::Item>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    pub payment: crate::generated::OrdersV2ResourcePayment,

    /// The details of the customer cost of shipping, including the customer chosen ShippingRate.
    pub shipping_cost: Option<crate::generated::OrdersV2ResourceShippingCost>,

    /// Customer shipping information associated with the order.
    pub shipping_details: Option<crate::generated::OrdersV2ResourceShippingDetails>,

    /// The overall status of the order.
    pub status: OrderStatus,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<crate::generated::OrdersV2ResourceTaxDetails>,

    pub total_details: crate::generated::OrdersV2ResourceTotalDetails,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParams {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostOrdersParamsAutomaticTax>,

    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PostOrdersParamsBillingDetails>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The coupons, promotion codes, and/or discounts to apply to the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostOrdersParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    pub line_items: Vec<PostOrdersParamsLineItems>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PostOrdersParamsPayment>,

    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<PostOrdersParamsShippingCost>,

    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<PostOrdersParamsShippingDetails>,

    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<PostOrdersParamsTaxDetails>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetOrdersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetOrdersIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParams {
    /// Settings for automatic tax calculation for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostOrdersIdParamsAutomaticTax>,

    /// Billing details for the customer.
    ///
    /// If a customer is provided, this will be automatically populated with values from that customer if override values are not provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PostOrdersIdParamsBillingDetails>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The customer associated with this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The coupons, promotion codes, and/or discounts to apply to the order.
    ///
    /// Pass the empty string `""` to unset this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostOrdersIdParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The IP address of the purchaser for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,

    /// A list of line items the customer is ordering.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<PostOrdersIdParamsLineItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Payment information associated with the order, including payment settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment: Option<PostOrdersIdParamsPayment>,

    /// Settings for the customer cost of shipping for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_cost: Option<PostOrdersIdParamsShippingCost>,

    /// Shipping details for the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_details: Option<PostOrdersIdParamsShippingDetails>,

    /// Additional tax details about the purchaser to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_details: Option<PostOrdersIdParamsTaxDetails>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdSubmitParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// `expected_total` should always be set to the order's `amount_total` field.
    ///
    /// If they don't match, submitting the order will fail.
    /// This helps detect race conditions where something else concurrently modifies the order.
    pub expected_total: i64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdReopenParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetOrdersIdLineItemsParams {
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
pub enum OrderStatus {
    Canceled,
    Complete,
    Open,
    Processing,
    Submitted,
}

impl OrderStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Complete => "complete",
            Self::Open => "open",
            Self::Processing => "processing",
            Self::Submitted => "submitted",
        }
    }
}

impl AsRef<str> for OrderStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrderStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrderStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
/// Settings for automatic tax calculation for this order.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsAutomaticTax {
    /// Enable automatic tax calculation which will automatically compute tax rates on this order.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsBillingDetails {
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostOrdersParamsBillingDetailsAddress>,

    /// The billing email provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The billing name provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The billing phone number provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,

    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsLineItems {
    /// The description for the line item.
    ///
    /// Will default to the name of the associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostOrdersParamsLineItemsDiscounts>>,

    /// The ID of a [Price](https://stripe.com/docs/api/prices) to add to the Order.
    ///
    /// The `price` parameter is an alternative to using the `product` parameter.
    ///
    /// If each of your products are sold at a single price, you can set `Product.default_price` and then pass the `product` parameter when creating a line item.
    /// If your products are sold at several possible prices, use the `price` parameter to explicitly specify which one to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new Price object inline.
    ///
    /// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
    ///
    /// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
    /// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
    /// This Price is hidden in both the Dashboard and API lists and cannot be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostOrdersParamsLineItemsPriceData>,

    /// The ID of a [Product](https://stripe.com/docs/api/products) to add to the Order.
    ///
    /// The product must have a `default_price` specified.
    ///
    /// Otherwise, specify the price by passing the `price` or `price_data` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Defines a Product inline and adds it to the Order.
    ///
    /// `product_data` is an alternative to the `product` parameter.
    ///
    /// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
    /// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<PostOrdersParamsLineItemsProductData>,

    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// Payment information associated with the order, including payment settings.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPayment {
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: PostOrdersParamsPaymentSettings,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<PostOrdersParamsShippingCostShippingRateData>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingDetails {
    /// The shipping address for the order.
    pub address: PostOrdersParamsShippingDetailsAddress,

    /// The name of the recipient of the order.
    pub name: String,

    /// The phone number (including extension) for the recipient of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Additional tax details about the purchaser to be used for this order.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsTaxDetails {
    /// The purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PostOrdersParamsTaxDetailsTaxExempt>,

    /// The purchaser's tax IDs to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<PostOrdersParamsTaxDetailsTaxIds>>,
}

/// Settings for automatic tax calculation for this order.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsAutomaticTax {
    /// Enable automatic tax calculation which will automatically compute tax rates on this order.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsBillingDetails {
    /// The billing address provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostOrdersIdParamsBillingDetailsAddress>,

    /// The billing email provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// The billing name provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The billing phone number provided by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,

    /// ID of the promotion code to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsLineItems {
    /// The description for the line item.
    ///
    /// Will default to the name of the associated product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostOrdersIdParamsLineItemsDiscounts>>,

    /// The ID of an existing line item on the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The ID of a [Price](https://stripe.com/docs/api/prices) to add to the Order.
    ///
    /// The `price` parameter is an alternative to using the `product` parameter.
    ///
    /// If each of your products are sold at a single price, you can set `Product.default_price` and then pass the `product` parameter when creating a line item.
    /// If your products are sold at several possible prices, use the `price` parameter to explicitly specify which one to use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new Price object inline.
    ///
    /// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
    ///
    /// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
    /// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
    /// This Price is hidden in both the Dashboard and API lists and cannot be reused.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostOrdersIdParamsLineItemsPriceData>,

    /// The ID of a [Product](https://stripe.com/docs/api/products) to add to the Order.
    ///
    /// The product must have a `default_price` specified.
    ///
    /// Otherwise, specify the price by passing the `price` or `price_data` parameter.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Defines a Product inline and adds it to the Order.
    ///
    /// `product_data` is an alternative to the `product` parameter.
    ///
    /// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
    /// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product_data: Option<PostOrdersIdParamsLineItemsProductData>,

    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates applied to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// Payment information associated with the order, including payment settings.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPayment {
    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: PostOrdersIdParamsPaymentSettings,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCost {
    /// The ID of the shipping rate to use for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate: Option<String>,

    /// Parameters to create a new ad-hoc shipping rate for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping_rate_data: Option<PostOrdersIdParamsShippingCostShippingRateData>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingDetails {
    /// The shipping address for the order.
    pub address: PostOrdersIdParamsShippingDetailsAddress,

    /// The name of the recipient of the order.
    pub name: String,

    /// The phone number (including extension) for the recipient of the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Additional tax details about the purchaser to be used for this order.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsTaxDetails {
    /// The purchaser's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PostOrdersIdParamsTaxDetailsTaxExempt>,

    /// The purchaser's tax IDs to be used for this order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<PostOrdersIdParamsTaxDetailsTaxIds>>,
}

/// The billing address provided by the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsLineItemsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// Data used to generate a new Price object inline.
///
/// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
///
/// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
/// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
/// This Price is hidden in both the Dashboard and API lists and cannot be reused.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// ID of the product this price belongs to.
    ///
    /// Use this to implement a variable-pricing model in your integration.
    ///
    /// This is required if `product_data` is not specifed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostOrdersParamsLineItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// Defines a Product inline and adds it to the Order.
///
/// `product_data` is an alternative to the `product` parameter.
///
/// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
/// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
/// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsLineItemsProductData {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A unique identifier for this product.
    ///
    /// `product_data` automatically creates a Product with this ID.
    ///
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    /// If any of the fields in the existing Product are different from the values in `product_data`, `product_data` updates the existing Product with the new information.
    /// So set `product_data[id]` to the same string every time you sell the same product, but don't re-use the same string for different products.
    pub id: String,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The product's name, meant to be displayable to the customer.
    pub name: String,

    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PostOrdersParamsLineItemsProductDataPackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Settings describing how the order should configure generated PaymentIntents.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettings {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptions>,

    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<PostOrdersParamsPaymentSettingsPaymentMethodTypes>>,

    /// The URL to redirect the customer to after they authenticate their payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// Provides configuration for completing a transfer for the order after it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostOrdersParamsPaymentSettingsTransferData>,
}

/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<PostOrdersParamsShippingCostShippingRateDataDeliveryEstimate>,

    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: String,

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<PostOrdersParamsShippingCostShippingRateDataFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostOrdersParamsShippingCostShippingRateDataTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostOrdersParamsShippingCostShippingRateDataType>,
}

/// The shipping address for the order.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PostOrdersParamsTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PostOrdersParamsTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsTaxDetailsTaxIds {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: PostOrdersParamsTaxDetailsTaxIdsType,

    /// Value of the tax ID.
    pub value: String,
}

/// The billing address provided by the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsBillingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsLineItemsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// Data used to generate a new Price object inline.
///
/// The `price_data` parameter is an alternative to using the `product` or `price` parameters.
///
/// If you create products upfront and configure a `Product.default_price`, pass the `product` parameter when creating a line item.
/// If you prefer not to define products upfront, or if you charge variable prices, pass the `price_data` parameter to describe the price for this line item.  Each time you pass `price_data` we create a Price for the product.
/// This Price is hidden in both the Dashboard and API lists and cannot be reused.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// ID of the product this price belongs to.
    ///
    /// Use this to implement a variable-pricing model in your integration.
    ///
    /// This is required if `product_data` is not specifed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub product: Option<String>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostOrdersIdParamsLineItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// Defines a Product inline and adds it to the Order.
///
/// `product_data` is an alternative to the `product` parameter.
///
/// If you created a Product upfront, use the `product` parameter to refer to the existing Product.
/// But if you prefer not to create Products upfront, pass the `product_data` parameter to define a Product inline as part of configuring the Order.  `product_data` automatically creates a Product, just as if you had manually created the Product.
/// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsLineItemsProductData {
    /// The product's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store a long form explanation of the product being sold for your own rendering purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// A unique identifier for this product.
    ///
    /// `product_data` automatically creates a Product with this ID.
    ///
    /// If a Product with the same ID already exists, then `product_data` re-uses it to avoid duplicates.
    /// If any of the fields in the existing Product are different from the values in `product_data`, `product_data` updates the existing Product with the new information.
    /// So set `product_data[id]` to the same string every time you sell the same product, but don't re-use the same string for different products.
    pub id: String,

    /// A list of up to 8 URLs of images for this product, meant to be displayable to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub images: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The product's name, meant to be displayable to the customer.
    pub name: String,

    /// The dimensions of this product for shipping purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub package_dimensions: Option<PostOrdersIdParamsLineItemsProductDataPackageDimensions>,

    /// Whether this product is shipped (i.e., physical goods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shippable: Option<bool>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// A URL of a publicly-accessible webpage for this product.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

/// Settings describing how the order should configure generated PaymentIntents.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettings {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptions>,

    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<PostOrdersIdParamsPaymentSettingsPaymentMethodTypes>>,

    /// The URL to redirect the customer to after they authenticate their payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub return_url: Option<String>,

    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor_suffix: Option<String>,

    /// Provides configuration for completing a transfer for the order after it is paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostOrdersIdParamsPaymentSettingsTransferData>,
}

/// Parameters to create a new ad-hoc shipping rate for this order.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateData {
    /// The estimated range for how long shipping will take, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delivery_estimate: Option<PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimate>,

    /// The name of the shipping rate, meant to be displayable to the customer.
    ///
    /// This will appear on CheckoutSessions.
    pub display_name: String,

    /// Describes a fixed amount to charge for shipping.
    ///
    /// Must be present if type is `fixed_amount`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amount: Option<PostOrdersIdParamsShippingCostShippingRateDataFixedAmount>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    ///
    /// The Shipping tax code is `txcd_92010001`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The type of calculation to use on the shipping rate.
    ///
    /// Can only be `fixed_amount` for now.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostOrdersIdParamsShippingCostShippingRateDataType>,
}

/// The shipping address for the order.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingDetailsAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State/province as an [ISO 3166-2](https://en.wikipedia.org/wiki/ISO_3166-2) subdivision code, without country prefix.
    ///
    /// Example: "NY" or "TX".
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsTaxDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PostOrdersIdParamsTaxDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsTaxDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsTaxDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsTaxDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsTaxDetailsTaxIds {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: PostOrdersIdParamsTaxDetailsTaxIdsType,

    /// Value of the tax ID.
    pub value: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersParamsLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostOrdersParamsLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsLineItemsProductDataPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,

    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,

    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,

    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}

/// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpay>,

    /// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipay>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdeal>,

    /// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarna>,

    /// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsLink>,

    /// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxo>,

    /// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24>,

    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebit>,

    /// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofort>,

    /// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}
/// Provides configuration for completing a transfer for the order after it is paid.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsTransferData {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of the Connected account receiving the transfer.
    pub destination: String,
}

/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximum>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersParamsShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostOrdersParamsShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsShippingCostShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsShippingCostShippingRateDataType {
    FixedAmount,
}

impl PostOrdersParamsShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for PostOrdersParamsShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsShippingCostShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsTaxDetailsTaxIdsType {
    AeTrn,
    AuAbn,
    AuArn,
    BgUic,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PhTin,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    ZaVat,
}

impl PostOrdersParamsTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AeTrn => "ae_trn",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::LiUid => "li_uid",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NoVat => "no_vat",
            Self::NzGst => "nz_gst",
            Self::PhTin => "ph_tin",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for PostOrdersParamsTaxDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsTaxDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsTaxDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersIdParamsLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsLineItemsProductDataPackageDimensions {
    /// Height, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub height: f64,

    /// Length, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub length: f64,

    /// Weight, in ounces.
    ///
    /// Maximum precision is 2 decimal places.
    pub weight: f64,

    /// Width, in inches.
    ///
    /// Maximum precision is 2 decimal places.
    pub width: f64,
}

/// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpay>,

    /// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipay>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdeal>,

    /// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarna>,

    /// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLink>,

    /// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxo>,

    /// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24>,

    /// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebit>,

    /// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofort>,

    /// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPay>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsTransferData {
    /// The amount that will be transferred automatically when the order is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of the Connected account receiving the transfer.
    pub destination: String,
}

/// The estimated range for how long shipping will take, meant to be displayable to the customer.
///
/// This will appear on CheckoutSessions.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub maximum: Option<PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximum>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum: Option<PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimum>,
}

/// Describes a fixed amount to charge for shipping.
///
/// Must be present if type is `fixed_amount`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateDataFixedAmount {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Shipping rates defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options:
        Option<PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsShippingCostShippingRateDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsShippingCostShippingRateDataType {
    FixedAmount,
}

impl PostOrdersIdParamsShippingCostShippingRateDataType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedAmount => "fixed_amount",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsShippingCostShippingRateDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsShippingCostShippingRateDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsShippingCostShippingRateDataType {
    fn default() -> Self {
        Self::FixedAmount
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsTaxDetailsTaxIdsType {
    AeTrn,
    AuAbn,
    AuArn,
    BgUic,
    BrCnpj,
    BrCpf,
    CaBn,
    CaGstHst,
    CaPstBc,
    CaPstMb,
    CaPstSk,
    CaQst,
    ChVat,
    ClTin,
    EgTin,
    EsCif,
    EuOssVat,
    EuVat,
    GbVat,
    GeVat,
    HkBr,
    HuTin,
    IdNpwp,
    IlVat,
    InGst,
    IsVat,
    JpCn,
    JpRn,
    JpTrn,
    KePin,
    KrBrn,
    LiUid,
    MxRfc,
    MyFrp,
    MyItn,
    MySst,
    NoVat,
    NzGst,
    PhTin,
    RuInn,
    RuKpp,
    SaVat,
    SgGst,
    SgUen,
    SiTin,
    ThVat,
    TrTin,
    TwVat,
    UaVat,
    UsEin,
    ZaVat,
}

impl PostOrdersIdParamsTaxDetailsTaxIdsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AeTrn => "ae_trn",
            Self::AuAbn => "au_abn",
            Self::AuArn => "au_arn",
            Self::BgUic => "bg_uic",
            Self::BrCnpj => "br_cnpj",
            Self::BrCpf => "br_cpf",
            Self::CaBn => "ca_bn",
            Self::CaGstHst => "ca_gst_hst",
            Self::CaPstBc => "ca_pst_bc",
            Self::CaPstMb => "ca_pst_mb",
            Self::CaPstSk => "ca_pst_sk",
            Self::CaQst => "ca_qst",
            Self::ChVat => "ch_vat",
            Self::ClTin => "cl_tin",
            Self::EgTin => "eg_tin",
            Self::EsCif => "es_cif",
            Self::EuOssVat => "eu_oss_vat",
            Self::EuVat => "eu_vat",
            Self::GbVat => "gb_vat",
            Self::GeVat => "ge_vat",
            Self::HkBr => "hk_br",
            Self::HuTin => "hu_tin",
            Self::IdNpwp => "id_npwp",
            Self::IlVat => "il_vat",
            Self::InGst => "in_gst",
            Self::IsVat => "is_vat",
            Self::JpCn => "jp_cn",
            Self::JpRn => "jp_rn",
            Self::JpTrn => "jp_trn",
            Self::KePin => "ke_pin",
            Self::KrBrn => "kr_brn",
            Self::LiUid => "li_uid",
            Self::MxRfc => "mx_rfc",
            Self::MyFrp => "my_frp",
            Self::MyItn => "my_itn",
            Self::MySst => "my_sst",
            Self::NoVat => "no_vat",
            Self::NzGst => "nz_gst",
            Self::PhTin => "ph_tin",
            Self::RuInn => "ru_inn",
            Self::RuKpp => "ru_kpp",
            Self::SaVat => "sa_vat",
            Self::SgGst => "sg_gst",
            Self::SgUen => "sg_uen",
            Self::SiTin => "si_tin",
            Self::ThVat => "th_vat",
            Self::TrTin => "tr_tin",
            Self::TwVat => "tw_vat",
            Self::UaVat => "ua_vat",
            Self::UsEin => "us_ein",
            Self::ZaVat => "za_vat",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsTaxDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsTaxDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsTaxDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}
/// If paying by `acss_debit`, this sub-hash contains details about the ACSS Debit payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

/// If paying by `afterpay_clearpay`, this sub-hash contains details about the AfterpayClearpay payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

/// If paying by `alipay`, this sub-hash contains details about the Alipay payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

/// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}

/// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}

/// If paying by `customer_balance`, this sub-hash contains details about the Customer Balance payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage>,
}

/// If paying by `ideal`, this sub-hash contains details about the iDEAL payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}

/// If paying by `klarna`, this sub-hash contains details about the Klarna payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,

    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

/// If paying by `link`, this sub-hash contains details about the Link payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,

    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}

/// If paying by `oxxo`, this sub-hash contains details about the OXXO payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

/// If paying by `p24`, this sub-hash contains details about the P24 payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,

    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

/// If paying by `sepa_debit`, this sub-hash contains details about the SEPA Debit payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

/// If paying by `sofort`, this sub-hash contains details about the Sofort payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}

/// If paying by `wechat_pay`, this sub-hash contains details about the WeChat Pay payment method options to pass to the order's PaymentIntent.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// The client type that the end customer will pay from.
    pub client: PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    /// Order identifier shown to the customer in Afterpay’s online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipay {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCard {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<
        PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdeal {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarna {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod>,

    /// Preferred language of the Klarna authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locale:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLink {
    /// Controls when the funds will be captured from the customer's account.
    ///
    /// If provided, this parameter will override the top-level `capture_method` when finalizing the payment with this payment method type.
    ///
    /// If `capture_method` is already set on the PaymentIntent, providing an empty value for this parameter will unset the stored value for this payment method type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod>,

    /// Token used for persistent Link logins.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub persistent_token: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxo {
    /// The number of calendar days before an OXXO voucher expires.
    ///
    /// For example, if you create an OXXO voucher on Monday and you set expires_after_days to 2, the OXXO invoice will expire on Wednesday at 23:59 America/Mexico_City time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_after_days: Option<u32>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24 {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage>,

    /// Confirm that the payer has accepted the P24 terms and conditions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tos_shown_and_accepted: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofort {
    /// Language shown to the payer on redirect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPay {
    /// The app ID registered with WeChat Pay.
    ///
    /// Only required when client is ios or android.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub app_id: Option<String>,

    /// The client type that the end customer will pay from.
    pub client: PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage:
        Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage>,
}

/// The upper bound of the estimated range.
///
/// If empty, represents no upper bound i.e., infinite.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximum {
    /// A unit of time.
    pub unit: PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

/// The lower bound of the estimated range.
///
/// If empty, represents no lower bound.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimum {
    /// A unit of time.
    pub unit: PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptions {
    /// A non-negative integer in cents representing how much to charge.
    pub amount: i64,

    /// Specifies whether the rate is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior>,
}

/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<Vec<PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaMinusDk,
    #[serde(rename = "de-AT")]
    DeMinusAt,
    #[serde(rename = "de-CH")]
    DeMinusCh,
    #[serde(rename = "de-DE")]
    DeMinusDe,
    #[serde(rename = "en-AT")]
    EnMinusAt,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-BE")]
    EnMinusBe,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-CH")]
    EnMinusCh,
    #[serde(rename = "en-DE")]
    EnMinusDe,
    #[serde(rename = "en-DK")]
    EnMinusDk,
    #[serde(rename = "en-ES")]
    EnMinusEs,
    #[serde(rename = "en-FI")]
    EnMinusFi,
    #[serde(rename = "en-FR")]
    EnMinusFr,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IT")]
    EnMinusIt,
    #[serde(rename = "en-NL")]
    EnMinusNl,
    #[serde(rename = "en-NO")]
    EnMinusNo,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-PL")]
    EnMinusPl,
    #[serde(rename = "en-PT")]
    EnMinusPt,
    #[serde(rename = "en-SE")]
    EnMinusSe,
    #[serde(rename = "en-US")]
    EnMinusUs,
    #[serde(rename = "es-ES")]
    EsMinusEs,
    #[serde(rename = "es-US")]
    EsMinusUs,
    #[serde(rename = "fi-FI")]
    FiMinusFi,
    #[serde(rename = "fr-BE")]
    FrMinusBe,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    #[serde(rename = "fr-CH")]
    FrMinusCh,
    #[serde(rename = "fr-FR")]
    FrMinusFr,
    #[serde(rename = "it-CH")]
    ItMinusCh,
    #[serde(rename = "it-IT")]
    ItMinusIt,
    #[serde(rename = "nb-NO")]
    NbMinusNo,
    #[serde(rename = "nl-BE")]
    NlMinusBe,
    #[serde(rename = "nl-NL")]
    NlMinusNl,
    #[serde(rename = "pl-PL")]
    PlMinusPl,
    #[serde(rename = "pt-PT")]
    PtMinusPt,
    #[serde(rename = "sv-FI")]
    SvMinusFi,
    #[serde(rename = "sv-SE")]
    SvMinusSe,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn default() -> Self {
        Self::DaMinusDk
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// A URL for custom mandate text to render during confirmation step.
    /// The URL will be rendered with additional GET parameters `payment_intent` and `payment_intent_client_secret` when confirming a Payment Intent,
    /// or `setup_intent` and `setup_intent_client_secret` when confirming a Setup Intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_mandate_url: Option<String>,

    /// Description of the mandate interval.
    ///
    /// Only required if 'payment_schedule' parameter is 'interval' or 'combined'.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_description: Option<String>,

    /// Payment schedule for the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_schedule: Option<
        PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule,
    >,

    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpayCaptureMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAfterpayClearpaySetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAlipaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Fr => "fr",
            Self::Nl => "nl",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsBancontactSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    Automatic,
    Manual,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCardSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
#[serde(skip_serializing_if = "Option::is_none")]
pub requested_address_types: Option<Vec<PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes>>,

    /// The list of bank transfer types that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
pub type_: PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    BankTransfer,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceSetupFutureUsage
{
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsIdealSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    Manual,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    #[serde(rename = "da-DK")]
    DaMinusDk,
    #[serde(rename = "de-AT")]
    DeMinusAt,
    #[serde(rename = "de-CH")]
    DeMinusCh,
    #[serde(rename = "de-DE")]
    DeMinusDe,
    #[serde(rename = "en-AT")]
    EnMinusAt,
    #[serde(rename = "en-AU")]
    EnMinusAu,
    #[serde(rename = "en-BE")]
    EnMinusBe,
    #[serde(rename = "en-CA")]
    EnMinusCa,
    #[serde(rename = "en-CH")]
    EnMinusCh,
    #[serde(rename = "en-DE")]
    EnMinusDe,
    #[serde(rename = "en-DK")]
    EnMinusDk,
    #[serde(rename = "en-ES")]
    EnMinusEs,
    #[serde(rename = "en-FI")]
    EnMinusFi,
    #[serde(rename = "en-FR")]
    EnMinusFr,
    #[serde(rename = "en-GB")]
    EnMinusGb,
    #[serde(rename = "en-IE")]
    EnMinusIe,
    #[serde(rename = "en-IT")]
    EnMinusIt,
    #[serde(rename = "en-NL")]
    EnMinusNl,
    #[serde(rename = "en-NO")]
    EnMinusNo,
    #[serde(rename = "en-NZ")]
    EnMinusNz,
    #[serde(rename = "en-PL")]
    EnMinusPl,
    #[serde(rename = "en-PT")]
    EnMinusPt,
    #[serde(rename = "en-SE")]
    EnMinusSe,
    #[serde(rename = "en-US")]
    EnMinusUs,
    #[serde(rename = "es-ES")]
    EsMinusEs,
    #[serde(rename = "es-US")]
    EsMinusUs,
    #[serde(rename = "fi-FI")]
    FiMinusFi,
    #[serde(rename = "fr-BE")]
    FrMinusBe,
    #[serde(rename = "fr-CA")]
    FrMinusCa,
    #[serde(rename = "fr-CH")]
    FrMinusCh,
    #[serde(rename = "fr-FR")]
    FrMinusFr,
    #[serde(rename = "it-CH")]
    ItMinusCh,
    #[serde(rename = "it-IT")]
    ItMinusIt,
    #[serde(rename = "nb-NO")]
    NbMinusNo,
    #[serde(rename = "nl-BE")]
    NlMinusBe,
    #[serde(rename = "nl-NL")]
    NlMinusNl,
    #[serde(rename = "pl-PL")]
    PlMinusPl,
    #[serde(rename = "pt-PT")]
    PtMinusPt,
    #[serde(rename = "sv-FI")]
    SvMinusFi,
    #[serde(rename = "sv-SE")]
    SvMinusSe,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DaMinusDk => "da-DK",
            Self::DeMinusAt => "de-AT",
            Self::DeMinusCh => "de-CH",
            Self::DeMinusDe => "de-DE",
            Self::EnMinusAt => "en-AT",
            Self::EnMinusAu => "en-AU",
            Self::EnMinusBe => "en-BE",
            Self::EnMinusCa => "en-CA",
            Self::EnMinusCh => "en-CH",
            Self::EnMinusDe => "en-DE",
            Self::EnMinusDk => "en-DK",
            Self::EnMinusEs => "en-ES",
            Self::EnMinusFi => "en-FI",
            Self::EnMinusFr => "en-FR",
            Self::EnMinusGb => "en-GB",
            Self::EnMinusIe => "en-IE",
            Self::EnMinusIt => "en-IT",
            Self::EnMinusNl => "en-NL",
            Self::EnMinusNo => "en-NO",
            Self::EnMinusNz => "en-NZ",
            Self::EnMinusPl => "en-PL",
            Self::EnMinusPt => "en-PT",
            Self::EnMinusSe => "en-SE",
            Self::EnMinusUs => "en-US",
            Self::EsMinusEs => "es-ES",
            Self::EsMinusUs => "es-US",
            Self::FiMinusFi => "fi-FI",
            Self::FrMinusBe => "fr-BE",
            Self::FrMinusCa => "fr-CA",
            Self::FrMinusCh => "fr-CH",
            Self::FrMinusFr => "fr-FR",
            Self::ItMinusCh => "it-CH",
            Self::ItMinusIt => "it-IT",
            Self::NbMinusNo => "nb-NO",
            Self::NlMinusBe => "nl-BE",
            Self::NlMinusNl => "nl-NL",
            Self::PlMinusPl => "pl-PL",
            Self::PtMinusPt => "pt-PT",
            Self::SvMinusFi => "sv-FI",
            Self::SvMinusSe => "sv-SE",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaPreferredLocale {
    fn default() -> Self {
        Self::DaMinusDk
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsKlarnaSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    Manual,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkCaptureMethod {
    fn default() -> Self {
        Self::Manual
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsLinkSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsOxxoSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsP24SetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitMandateOptions {}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    None,
    OffSession,
    OnSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
            Self::OnSession => "on_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSepaDebitSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    De,
    En,
    Es,
    Fr,
    It,
    Nl,
    Pl,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "de",
            Self::En => "en",
            Self::Es => "es",
            Self::Fr => "fr",
            Self::It => "it",
            Self::Nl => "nl",
            Self::Pl => "pl",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    None,
    OffSession,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::OffSession => "off_session",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsSofortSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    Android,
    Ios,
    Web,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Android => "android",
            Self::Ios => "ios",
            Self::Web => "web",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPayClient {
    fn default() -> Self {
        Self::Android
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    None,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsWechatPaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMaximumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsShippingCostShippingRateDataDeliveryEstimateMinimumUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsShippingCostShippingRateDataFixedAmountCurrencyOptionsTaxBehavior
{
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    Combined,
    Interval,
    Sporadic,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    Business,
    Personal,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Iban => "iban",
Self::Sepa => "sepa",
Self::SortCode => "sort_code",
Self::Spei => "spei",
Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str>
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    Combined,
    Interval,
    Sporadic,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Combined => "combined",
            Self::Interval => "interval",
            Self::Sporadic => "sporadic",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsPaymentSchedule
{
    fn default() -> Self {
        Self::Combined
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes
{
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Iban => "iban",
Self::Sepa => "sepa",
Self::SortCode => "sort_code",
Self::Spei => "spei",
Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str>
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostOrdersIdParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferType
{
    fn default() -> Self {
        Self::EuBankTransfer
    }
}
pub fn post_orders(
    client: &crate::Client,
    params: PostOrdersParams,
) -> crate::Response<crate::generated::Order> {
    client.post_form("/orders", params)
}

pub fn get_orders(
    client: &crate::Client,
    params: GetOrdersParams,
) -> crate::Response<crate::params::List<crate::generated::Order>> {
    client.get_query("/orders", params)
}

pub fn get_orders_id(
    client: &crate::Client,
    id: String,
    params: GetOrdersIdParams,
) -> crate::Response<crate::generated::Order> {
    client.get_query(&format!("/orders/{id}", id = id), params)
}

pub fn post_orders_id(
    client: &crate::Client,
    id: String,
    params: PostOrdersIdParams,
) -> crate::Response<crate::generated::Order> {
    client.post_form(&format!("/orders/{id}", id = id), params)
}

pub fn post_orders_id_submit(
    client: &crate::Client,
    id: String,
    params: PostOrdersIdSubmitParams,
) -> crate::Response<crate::generated::Order> {
    client.post_form(&format!("/orders/{id}/submit", id = id), params)
}

pub fn post_orders_id_cancel(
    client: &crate::Client,
    id: String,
    params: PostOrdersIdCancelParams,
) -> crate::Response<crate::generated::Order> {
    client.post_form(&format!("/orders/{id}/cancel", id = id), params)
}

pub fn post_orders_id_reopen(
    client: &crate::Client,
    id: String,
    params: PostOrdersIdReopenParams,
) -> crate::Response<crate::generated::Order> {
    client.post_form(&format!("/orders/{id}/reopen", id = id), params)
}

pub fn get_orders_id_line_items(
    client: &crate::Client,
    id: String,
    params: GetOrdersIdLineItemsParams,
) -> crate::Response<crate::params::List<crate::generated::Item>> {
    client.get_query(&format!("/orders/{id}/line_items", id = id), params)
}
