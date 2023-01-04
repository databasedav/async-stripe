/// A Quote is a way to model prices that you'd like to provide to a customer.
/// Once accepted, it will automatically create an invoice, subscription or subscription schedule.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Quote {
    /// Total before any discounts or taxes are applied.
    pub amount_subtotal: i64,

    /// Total after discounts and taxes are applied.
    pub amount_total: i64,

    /// ID of the Connect Application that created the quote.
    pub application: Option<Vec<crate::generated::Application>>,

    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// Only applicable if there are no line items with recurring prices on the quote.
    pub application_fee_amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// Only applicable if there are line items with recurring prices on the quote.
    pub application_fee_percent: Option<f64>,

    pub automatic_tax: crate::generated::QuotesResourceAutomaticTax,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or on finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    pub collection_method: QuoteCollectionMethod,

    pub computed: crate::generated::QuotesResourceComputed,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: Option<crate::currency::Currency>,

    /// The customer which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// The tax rates applied to this quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<Vec<crate::generated::TaxRate>>>,

    /// A description that will be displayed on the quote PDF.
    pub description: Option<String>,

    /// The discounts applied to this quote.
    pub discounts: Vec<Vec<crate::generated::Discount>>,

    /// The date on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    pub expires_at: crate::resources::Scheduled,

    /// A footer that will be displayed on the quote PDF.
    pub footer: Option<String>,

    /// Details of the quote that was cloned.
    ///
    /// See the [cloning documentation](https://stripe.com/docs/quotes/clone) for more details.
    pub from_quote: Option<crate::generated::QuotesResourceFromQuote>,

    /// A header that will be displayed on the quote PDF.
    pub header: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// The invoice that was created from this quote.
    pub invoice: Option<Vec<crate::generated::Invoice>>,

    /// All invoices will be billed using the specified settings.
    pub invoice_settings: Option<crate::generated::InvoiceSettingQuoteSetting>,

    /// A list of items the customer is being quoted for.
    #[serde(default)]
    pub line_items: crate::params::List<crate::generated::Item>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// A unique number that identifies this particular quote.
    ///
    /// This number is assigned once the quote is [finalized](https://stripe.com/docs/quotes/overview#finalize).
    pub number: Option<String>,

    /// The account on behalf of which to charge.
    ///
    /// See the [Connect documentation](https://support.stripe.com/questions/sending-invoices-on-behalf-of-connected-accounts) for details.
    pub on_behalf_of: Option<Vec<crate::generated::Account>>,

    /// The status of the quote.
    pub status: QuoteStatus,

    pub status_transitions: crate::generated::QuotesResourceStatusTransitions,

    /// The subscription that was created or updated from this quote.
    pub subscription: Option<Vec<crate::generated::Subscription>>,

    pub subscription_data: crate::generated::QuotesResourceSubscriptionData,

    /// The subscription schedule that was created or updated from this quote.
    pub subscription_schedule: Option<Vec<crate::generated::SubscriptionSchedule>>,

    /// ID of the test clock this quote belongs to.
    pub test_clock: Option<Vec<crate::generated::TestHelpersTestClock>>,

    pub total_details: crate::generated::QuotesResourceTotalDetails,

    /// The account (if any) the payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the invoices.
    pub transfer_data: Option<crate::generated::QuotesResourceTransferData>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetQuotesQuoteParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParams {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostQuotesParamsAutomaticTax>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostQuotesParamsCollectionMethod>,

    /// The customer for which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// A description that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default description configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts applied to the quote.
    ///
    /// You can only set up to one discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostQuotesParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    /// If no value is passed, the default expiration date configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A footer that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default footer configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Clone an existing quote.
    ///
    /// The new quote will be created in `status=draft`.
    /// When using this parameter, you cannot specify any other parameters except for `expires_at`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_quote: Option<PostQuotesParamsFromQuote>,

    /// A header that will be displayed on the quote PDF.
    ///
    /// If no value is passed, the default header configured in your [quote template settings](https://dashboard.stripe.com/settings/billing/quote) will be used.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostQuotesParamsInvoiceSettings>,

    /// A list of line items the customer is being quoted for.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<PostQuotesParamsLineItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<PostQuotesParamsSubscriptionData>,

    /// ID of the test clock to attach to the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,

    /// The data with which to automatically create a Transfer for each of the invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostQuotesParamsTransferData>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParams {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    ///
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostQuotesQuoteParamsAutomaticTax>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay invoices at the end of the subscription cycle or at invoice finalization using the default payment method attached to the subscription or customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostQuotesQuoteParamsCollectionMethod>,

    /// The customer for which this quote belongs to.
    ///
    /// A customer is required before finalizing the quote.
    /// Once specified, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// A description that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts applied to the quote.
    ///
    /// You can only set up to one discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostQuotesQuoteParamsDiscounts>>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A footer that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// A header that will be displayed on the quote PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostQuotesQuoteParamsInvoiceSettings>,

    /// A list of line items the customer is being quoted for.
    ///
    /// Each line item includes information about the product, the quantity, and the resulting cost.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line_items: Option<Vec<PostQuotesQuoteParamsLineItems>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account on behalf of which to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// When creating a subscription or subscription schedule, the specified configuration data will be used.
    ///
    /// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
    /// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_data: Option<PostQuotesQuoteParamsSubscriptionData>,

    /// The data with which to automatically create a Transfer for each of the invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostQuotesQuoteParamsTransferData>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteFinalizeParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A future timestamp on which the quote will be canceled if in `open` or `draft` status.
    ///
    /// Measured in seconds since the Unix epoch.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteAcceptParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetQuotesParams {
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

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetQuotesParamsStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetQuotesQuoteLineItemsParams {
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
pub struct GetQuotesQuoteComputedUpfrontLineItemsParams {
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
pub enum QuoteCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl QuoteCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for QuoteCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuoteCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for QuoteCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuoteStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

impl QuoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Canceled => "canceled",
            Self::Draft => "draft",
            Self::Open => "open",
        }
    }
}

impl AsRef<str> for QuoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for QuoteStatus {
    fn default() -> Self {
        Self::Accepted
    }
}
/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostQuotesParamsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostQuotesParamsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesParamsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesParamsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// Clone an existing quote.
///
/// The new quote will be created in `status=draft`.
/// When using this parameter, you cannot specify any other parameters except for `expires_at`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsFromQuote {
    /// Whether this quote is a revision of the previous quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_revision: Option<bool>,

    /// The `id` of the quote that will be cloned.
    pub quote: String,
}

/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    ///
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsLineItems {
    /// The ID of the price object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostQuotesParamsLineItemsPriceData>,

    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the line item.
    ///
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// When creating a subscription or subscription schedule, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<PostQuotesParamsSubscriptionDataEffectiveDate>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

/// Settings for automatic tax lookup for this quote and resulting invoices and subscriptions.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsAutomaticTax {
    /// Controls whether Stripe will automatically compute tax on the resulting invoices or subscriptions as well as the quote itself.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesQuoteParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostQuotesQuoteParamsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostQuotesQuoteParamsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesQuoteParamsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesQuoteParamsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsInvoiceSettings {
    /// Number of days within which a customer must pay the invoice generated by this quote.
    ///
    /// This value will be `null` for quotes where `collection_method=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsLineItems {
    /// The ID of an existing line item on the quote.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The ID of the price object.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    ///
    /// One of `price` or `price_data` is required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostQuotesQuoteParamsLineItemsPriceData>,

    /// The quantity of the line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the line item.
    ///
    /// When set, the `default_tax_rates` on the quote do not apply to this line item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// When creating a subscription or subscription schedule, the specified configuration data will be used.
///
/// There must be at least one line item with a recurring price for a subscription or subscription schedule to be created.
/// A subscription schedule is created if `subscription_data[effective_date]` is present and in the future, otherwise a subscription is created.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsSubscriptionData {
    /// The subscription's description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
    ///
    /// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
    /// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
    /// The `effective_date` is ignored if it is in the past when the quote is accepted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_date: Option<PostQuotesQuoteParamsSubscriptionDataEffectiveDate>,

    /// Integer representing the number of trial period days before the customer is charged for the first time.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_period_days: Option<u32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    /// There cannot be any line items with recurring prices when using this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    /// There must be at least 1 line item with a recurring price to use this field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetQuotesParamsStatus {
    Accepted,
    Canceled,
    Draft,
    Open,
}

impl GetQuotesParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Canceled => "canceled",
            Self::Draft => "draft",
            Self::Open => "open",
        }
    }
}

impl AsRef<str> for GetQuotesParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetQuotesParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetQuotesParamsStatus {
    fn default() -> Self {
        Self::Accepted
    }
}
/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
///
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<PostQuotesParamsLineItemsPriceDataRecurring>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostQuotesParamsLineItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
///
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostQuotesParamsSubscriptionDataEffectiveDate {
    CurrentPeriodEnd(PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd),
    Timestamp(crate::params::Timestamp),
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
///
/// One of `price` or `price_data` is required.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsLineItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub recurring: Option<PostQuotesQuoteParamsLineItemsPriceDataRecurring>,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// When creating a new subscription, the date of which the subscription schedule will start after the quote is accepted.
///
/// When updating a subscription, the date of which the subscription will be updated using a subscription schedule.
/// The special value `current_period_end` can be provided to update a subscription at the end of its current period.
/// The `effective_date` is ignored if it is in the past when the quote is accepted.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostQuotesQuoteParamsSubscriptionDataEffectiveDate {
    CurrentPeriodEnd(PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd),
    Timestamp(crate::params::Timestamp),
}

/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesParamsLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PostQuotesParamsLineItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesParamsLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostQuotesParamsLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostQuotesParamsLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesParamsLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesParamsLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    CurrentPeriodEnd,
}

impl PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentPeriodEnd => "current_period_end",
        }
    }
}

impl AsRef<str> for PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn default() -> Self {
        Self::CurrentPeriodEnd
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostQuotesQuoteParamsLineItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesQuoteParamsLineItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    CurrentPeriodEnd,
}

impl PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CurrentPeriodEnd => "current_period_end",
        }
    }
}

impl AsRef<str> for PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesQuoteParamsSubscriptionDataEffectiveDateCurrentPeriodEnd {
    fn default() -> Self {
        Self::CurrentPeriodEnd
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesParamsLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PostQuotesParamsLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for PostQuotesParamsLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesParamsLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesParamsLineItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostQuotesQuoteParamsLineItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}
pub fn get_quotes_quote(
    client: &crate::Client,
    quote: String,
    params: GetQuotesQuoteParams,
) -> crate::Response<crate::generated::Quote> {
    client.get_query(&format!("/quotes/{quote}", quote = quote), params)
}

pub fn post_quotes(
    client: &crate::Client,
    params: PostQuotesParams,
) -> crate::Response<crate::generated::Quote> {
    client.post_form("/quotes", params)
}

pub fn post_quotes_quote(
    client: &crate::Client,
    quote: String,
    params: PostQuotesQuoteParams,
) -> crate::Response<crate::generated::Quote> {
    client.post_form(&format!("/quotes/{quote}", quote = quote), params)
}

pub fn post_quotes_quote_cancel(
    client: &crate::Client,
    quote: String,
    params: PostQuotesQuoteCancelParams,
) -> crate::Response<crate::generated::Quote> {
    client.post_form(&format!("/quotes/{quote}/cancel", quote = quote), params)
}

pub fn post_quotes_quote_finalize(
    client: &crate::Client,
    quote: String,
    params: PostQuotesQuoteFinalizeParams,
) -> crate::Response<crate::generated::Quote> {
    client.post_form(&format!("/quotes/{quote}/finalize", quote = quote), params)
}

pub fn post_quotes_quote_accept(
    client: &crate::Client,
    quote: String,
    params: PostQuotesQuoteAcceptParams,
) -> crate::Response<crate::generated::Quote> {
    client.post_form(&format!("/quotes/{quote}/accept", quote = quote), params)
}

pub fn get_quotes(
    client: &crate::Client,
    params: GetQuotesParams,
) -> crate::Response<crate::params::List<crate::generated::Quote>> {
    client.get_query("/quotes", params)
}

pub fn get_quotes_quote_line_items(
    client: &crate::Client,
    quote: String,
    params: GetQuotesQuoteLineItemsParams,
) -> crate::Response<crate::params::List<crate::generated::Item>> {
    client.get_query(&format!("/quotes/{quote}/line_items", quote = quote), params)
}

pub fn get_quotes_quote_computed_upfront_line_items(
    client: &crate::Client,
    quote: String,
    params: GetQuotesQuoteComputedUpfrontLineItemsParams,
) -> crate::Response<crate::params::List<crate::generated::Item>> {
    client.get_query(&format!("/quotes/{quote}/computed_upfront_line_items", quote = quote), params)
}
