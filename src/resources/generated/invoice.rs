/// Invoices are statements of amounts owed by a customer, and are either
/// generated one-off, or generated periodically from a subscription.
///
/// They contain [invoice items](https://stripe.com/docs/api#invoiceitems), and proration adjustments
/// that may be caused by subscription upgrades/downgrades (if necessary).
///
/// If your invoice is configured to be billed through automatic charges,
/// Stripe automatically finalizes your invoice and attempts payment.
///
/// Note that finalizing the invoice, [when automatic](https://stripe.com/docs/billing/invoices/workflow/#auto_advance), does not happen immediately as the invoice is created.
/// Stripe waits until one hour after the last webhook was successfully sent (or the last webhook timed out after failing).
/// If you (and the platforms you may have connected to) have no webhooks configured, Stripe waits one hour after creation to finalize the invoice.  If your invoice is configured to be billed by sending an email, then based on your [email settings](https://dashboard.stripe.com/account/billing/automatic), Stripe will email the invoice to your customer and await payment.
/// These emails can contain a link to a hosted page to pay the invoice.  Stripe applies any customer credit on the account before determining the amount due for the invoice (i.e., the amount that will be actually charged).
/// If the amount due for the invoice is less than Stripe's [minimum allowed charge per currency](/docs/currencies#minimum-and-maximum-charge-amounts), the invoice is automatically marked paid, and we add the amount due to the customer's credit balance which is applied to the next invoice.  More details on the customer's credit balance are [here](https://stripe.com/docs/billing/customer/balance).  Related guide: [Send Invoices to Customers](https://stripe.com/docs/billing/invoices/sending).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Invoice {
    /// The country of the business associated with this invoice, most often the business creating the invoice.
    pub account_country: Option<String>,

    /// The public name of the business associated with this invoice, most often the business creating the invoice.
    pub account_name: Option<String>,

    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    pub account_tax_ids: Option<Vec<Vec<crate::generated::TaxId>>>,

    /// Final amount due at this time for this invoice.
    ///
    /// If the invoice's total is smaller than the minimum charge amount, for example, or if there is account credit that can be applied to the invoice, the `amount_due` may be 0.
    /// If there is a positive `starting_balance` for the invoice (the customer owes money), the `amount_due` will also take that into account.
    /// The charge that gets generated for the invoice will be for the amount specified in `amount_due`.
    pub amount_due: i64,

    /// The amount, in %s, that was paid.
    pub amount_paid: i64,

    /// The difference between amount_due and amount_paid, in %s.
    pub amount_remaining: i64,

    /// ID of the Connect Application that created the invoice.
    pub application: Option<Vec<crate::generated::Application>>,

    /// The fee in %s that will be applied to the invoice and transferred to the application owner's Stripe account when the invoice is paid.
    pub application_fee_amount: Option<i64>,

    /// Number of payment attempts made for this invoice, from the perspective of the payment retry schedule.
    ///
    /// Any payment attempt counts as the first attempt, and subsequently only automatic retries increment the attempt count.
    /// In other words, manual payment attempts after the first attempt do not affect the retry schedule.
    pub attempt_count: u64,

    /// Whether an attempt has been made to pay the invoice.
    ///
    /// An invoice is not attempted until 1 hour after the `invoice.created` webhook, for example, so you might not want to display that invoice as unpaid to your users.
    pub attempted: bool,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    pub automatic_tax: crate::generated::AutomaticTax,

    /// Indicates the reason why the invoice was created.
    ///
    /// `subscription_cycle` indicates an invoice created by a subscription advancing into a new period.
    /// `subscription_create` indicates an invoice created due to creating a subscription.
    /// `subscription_update` indicates an invoice created due to updating a subscription.
    /// `subscription` is set for all old invoices to indicate either a change to a subscription or a period advancement.
    /// `manual` is set for all invoices unrelated to a subscription (for example: created via the invoice editor).
    /// The `upcoming` value is reserved for simulated invoices per the upcoming invoice endpoint.
    /// `subscription_threshold` indicates an invoice created due to a billing threshold being reached.
    pub billing_reason: Option<InvoiceBillingReason>,

    /// ID of the latest charge generated for this invoice, if any.
    pub charge: Option<Vec<crate::generated::Charge>>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    pub collection_method: InvoiceCollectionMethod,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Custom fields displayed on the invoice.
    pub custom_fields: Option<Vec<crate::generated::InvoiceSettingCustomField>>,

    /// The ID of the customer who will be billed.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// The customer's address.
    ///
    /// Until the invoice is finalized, this field will equal `customer.address`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_address: Option<crate::generated::Address>,

    /// The customer's email.
    ///
    /// Until the invoice is finalized, this field will equal `customer.email`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_email: Option<String>,

    /// The customer's name.
    ///
    /// Until the invoice is finalized, this field will equal `customer.name`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_name: Option<String>,

    /// The customer's phone number.
    ///
    /// Until the invoice is finalized, this field will equal `customer.phone`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_phone: Option<String>,

    /// The customer's shipping information.
    ///
    /// Until the invoice is finalized, this field will equal `customer.shipping`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_shipping: Option<crate::generated::Shipping>,

    /// The customer's tax exempt status.
    ///
    /// Until the invoice is finalized, this field will equal `customer.tax_exempt`.
    /// Once the invoice is finalized, this field will no longer be updated.
    pub customer_tax_exempt: Option<InvoiceCustomerTaxExempt>,

    /// The customer's tax IDs.
    ///
    /// Until the invoice is finalized, this field will contain the same tax IDs as `customer.tax_ids`.
    /// Once the invoice is finalized, this field will no longer be updated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_tax_ids: Option<Vec<crate::generated::InvoicesResourceInvoiceTaxId>>,

    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<Vec<crate::generated::PaymentMethod>>,

    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    pub default_source: Option<Vec<crate::generated::PaymentSource>>,

    /// The tax rates applied to this invoice, if any.
    pub default_tax_rates: Vec<crate::generated::TaxRate>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    pub description: Option<String>,

    /// Describes the current discount applied to this invoice, if there is one.
    ///
    /// Not populated if there are multiple discounts.
    pub discount: Option<crate::generated::Discount>,

    /// The discounts applied to the invoice.
    ///
    /// Line item discounts are applied before invoice discounts.
    /// Use `expand[]=discounts` to expand each discount.
    pub discounts: Option<Vec<Vec<crate::generated::Discount>>>,

    /// The date on which payment for this invoice is due.
    ///
    /// This value will be `null` for invoices where `collection_method=charge_automatically`.
    pub due_date: Option<crate::params::Timestamp>,

    /// Ending customer balance after the invoice is finalized.
    ///
    /// Invoices are finalized approximately an hour after successful webhook delivery or when payment collection is attempted for the invoice.
    /// If the invoice has not been finalized yet, this will be null.
    pub ending_balance: Option<i64>,

    /// Footer displayed on the invoice.
    pub footer: Option<String>,

    /// Details of the invoice that was cloned.
    ///
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    pub from_invoice: Option<crate::generated::InvoicesFromInvoice>,

    /// The URL for the hosted invoice page, which allows customers to view and pay an invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hosted_invoice_url: Option<String>,

    /// Unique identifier for the object.
    ///
    /// This property is always present unless the invoice is an upcoming invoice.
    /// See [Retrieve an upcoming invoice](https://stripe.com/docs/api/invoices/upcoming) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// The link to download the PDF for the invoice.
    ///
    /// If the invoice has not been finalized yet, this will be null.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_pdf: Option<String>,

    /// The error encountered during the previous attempt to finalize the invoice.
    ///
    /// This field is cleared when the invoice is successfully finalized.
    pub last_finalization_error: Option<Box<crate::generated::ApiErrors>>,

    /// The ID of the most recent non-draft revision of this invoice.
    pub latest_revision: Option<Vec<crate::generated::Invoice>>,

    /// The individual line items that make up the invoice.
    ///
    /// `lines` is sorted as follows: invoice items in reverse chronological order, followed by the subscription, if any.
    pub lines: crate::params::List<crate::generated::LineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// The time at which payment will next be attempted.
    ///
    /// This value will be `null` for invoices where `collection_method=send_invoice`.
    pub next_payment_attempt: Option<crate::params::Timestamp>,

    /// A unique, identifying string that appears on emails sent to the customer for this invoice.
    ///
    /// This starts with the customer's unique invoice_prefix if it is specified.
    pub number: Option<String>,

    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    pub on_behalf_of: Option<Vec<crate::generated::Account>>,

    /// Whether payment was successfully collected for this invoice.
    ///
    /// An invoice can be paid (most commonly) with a charge or with credit from the customer's account balance.
    pub paid: bool,

    /// Returns true if the invoice was manually marked paid, returns false if the invoice hasn't been paid yet or was paid on Stripe.
    pub paid_out_of_band: bool,

    /// The PaymentIntent associated with this invoice.
    ///
    /// The PaymentIntent is generated when the invoice is finalized, and can then be used to pay the invoice.
    /// Note that voiding an invoice will cancel the PaymentIntent.
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,

    pub payment_settings: crate::generated::InvoicesPaymentSettings,

    /// End of the usage period during which invoice items were added to this invoice.
    pub period_end: crate::params::Timestamp,

    /// Start of the usage period during which invoice items were added to this invoice.
    pub period_start: crate::params::Timestamp,

    /// Total amount of all post-payment credit notes issued for this invoice.
    pub post_payment_credit_notes_amount: i64,

    /// Total amount of all pre-payment credit notes issued for this invoice.
    pub pre_payment_credit_notes_amount: i64,

    /// The quote this invoice was generated from.
    pub quote: Option<Vec<crate::generated::Quote>>,

    /// This is the transaction number that appears on email receipts sent for this invoice.
    pub receipt_number: Option<String>,

    /// Options for invoice PDF rendering.
    pub rendering_options: Option<crate::generated::InvoiceSettingRenderingOptions>,

    /// Starting customer balance before the invoice is finalized.
    ///
    /// If the invoice has not been finalized yet, this will be the current customer balance.
    /// For revision invoices, this also includes any customer balance that was applied to the original invoice.
    pub starting_balance: i64,

    /// Extra information about an invoice for the customer's credit card statement.
    pub statement_descriptor: Option<String>,

    /// The status of the invoice, one of `draft`, `open`, `paid`, `uncollectible`, or `void`.
    ///
    /// [Learn more](https://stripe.com/docs/billing/invoices/workflow#workflow-overview).
    pub status: Option<InvoiceStatus>,

    pub status_transitions: crate::generated::InvoicesStatusTransitions,

    /// The subscription that this invoice was prepared for, if any.
    pub subscription: Option<Vec<crate::generated::Subscription>>,

    /// Only set for upcoming invoices that preview prorations.
    ///
    /// The time used to calculate prorations.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<crate::params::Timestamp>,

    /// Total of all subscriptions, invoice items, and prorations on the invoice before any invoice level discount or exclusive tax is applied.
    ///
    /// Item discounts are already incorporated.
    pub subtotal: i64,

    /// The integer amount in %s representing the subtotal of the invoice before any invoice level discount or tax is applied.
    ///
    /// Item discounts are already incorporated.
    pub subtotal_excluding_tax: Option<i64>,

    /// The amount of tax on this invoice.
    ///
    /// This is the sum of all the tax amounts on this invoice.
    pub tax: Option<i64>,

    /// ID of the test clock this invoice belongs to.
    pub test_clock: Option<Vec<crate::generated::TestHelpersTestClock>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub threshold_reason: Option<crate::generated::InvoiceThresholdReason>,

    /// Total after discounts and taxes.
    pub total: i64,

    /// The aggregate amounts calculated per discount across all line items.
    pub total_discount_amounts: Option<Vec<crate::generated::DiscountsResourceDiscountAmount>>,

    /// The integer amount in %s representing the total amount of the invoice including all discounts but excluding all tax.
    pub total_excluding_tax: Option<i64>,

    /// The aggregate amounts calculated per tax rate for all line items.
    pub total_tax_amounts: Vec<crate::generated::InvoiceTaxAmount>,

    /// The account (if any) the payment will be attributed to for tax reporting, and where funds from the payment will be transferred to for the invoice.
    pub transfer_data: Option<crate::generated::InvoiceTransferData>,

    /// Invoices are automatically paid or sent 1 hour after webhooks are delivered, or until all webhook delivery attempts have [been exhausted](https://stripe.com/docs/billing/webhooks#understand).
    ///
    /// This field tracks the time when webhooks for this invoice were successfully delivered.
    /// If the invoice had no webhooks to deliver, this will be set while the invoice is being created.
    pub webhooks_delivered_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ReturnedGetInvoicesSearch {
    pub data: Vec<crate::generated::Invoice>,

    pub has_more: bool,

    pub next_page: Option<String>,

    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReturnedGetInvoicesSearchObject,

    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,

    pub url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    pub query: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<GetInvoicesParamsCollectionMethod>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetInvoicesParamsStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<GetInvoicesUpcomingParamsAutomaticTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<GetInvoicesUpcomingParamsCustomerDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<GetInvoicesUpcomingParamsDiscounts>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_items: Option<Vec<GetInvoicesUpcomingParamsInvoiceItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_billing_cycle_anchor:
        Option<GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at_period_end: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_now: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<Vec<GetInvoicesUpcomingParamsSubscriptionItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_behavior:
        Option<GetInvoicesUpcomingParamsSubscriptionProrationBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<GetInvoicesUpcomingParamsSubscriptionTrialEnd>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_from_plan: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParams {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostInvoicesParamsAutomaticTax>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay this invoice using the default source attached to the customer.
    /// When sending an invoice, Stripe will email this invoice to the customer with payment instructions.
    /// Defaults to `charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostInvoicesParamsCollectionMethod>,

    /// The currency to create this invoice in.
    ///
    /// Defaults to that of `customer` if not specified.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<PostInvoicesParamsCustomFields>>,

    /// The ID of the customer who will be billed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// The number of days from when the invoice is created until it is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,

    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The coupons to redeem into discounts for the invoice.
    ///
    /// If not specified, inherits the discount from the invoice's customer.
    /// Pass an empty string to avoid inheriting any discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostInvoicesParamsDiscounts>>,

    /// The date on which payment for this invoice is due.
    ///
    /// Valid only for invoices where `collection_method=send_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<crate::params::Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Revise an existing invoice.
    ///
    /// The new invoice will be created in `status=draft`.
    /// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_invoice: Option<PostInvoicesParamsFromInvoice>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<PostInvoicesParamsPaymentSettings>,

    /// How to handle pending invoice items on invoice creation.
    ///
    /// One of `include` or `exclude`.
    /// `include` will include any pending invoice items, and will create an empty draft invoice if no pending invoice items exist.
    /// `exclude` will always create an empty invoice draft regardless if there are pending invoice items or not.
    /// Defaults to `exclude` if the parameter is omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_invoice_items_behavior: Option<PostInvoicesParamsPendingInvoiceItemsBehavior>,

    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<PostInvoicesParamsRenderingOptions>,

    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// The ID of the subscription to invoice, if any.
    ///
    /// If set, the created invoice will only include pending invoice items for that subscription and pending invoice items not associated with any subscription if `pending_invoice_items_behavior` is `include`.
    /// The subscription's billing cycle and regular subscription events won't be affected.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostInvoicesParamsTransferData>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesInvoiceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParams {
    /// The account tax IDs associated with the invoice.
    ///
    /// Only editable when the invoice is a draft.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_tax_ids: Option<Vec<String>>,

    /// A fee in cents (or local equivalent) that will be applied to the invoice and transferred to the application owner's Stripe account.
    ///
    /// The request must be made with an OAuth key or the Stripe-Account header in order to take an application fee.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/billing/invoices/connect#collecting-fees).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_amount: Option<i64>,

    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/billing/invoices/workflow/#auto_advance) of the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Settings for automatic tax lookup for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostInvoicesInvoiceParamsAutomaticTax>,

    /// Either `charge_automatically` or `send_invoice`.
    ///
    /// This field can be updated only on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostInvoicesInvoiceParamsCollectionMethod>,

    /// A list of up to 4 custom fields to be displayed on the invoice.
    ///
    /// If a value for `custom_fields` is specified, the list specified will replace the existing custom field list on this invoice.
    /// Pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<PostInvoicesInvoiceParamsCustomFields>>,

    /// The number of days from which the invoice is created until it is due.
    ///
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,

    /// ID of the default payment method for the invoice.
    ///
    /// It must belong to the customer associated with the invoice.
    /// If not set, defaults to the subscription's default payment method, if any, or to the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// ID of the default payment source for the invoice.
    ///
    /// It must belong to the customer associated with the invoice and be in a chargeable state.
    /// If not set, defaults to the subscription's default source, if any, or to the customer's default source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,

    /// The tax rates that will apply to any line item that does not have `tax_rates` set.
    ///
    /// Pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    /// Referenced as 'memo' in the Dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The discounts that will apply to the invoice.
    ///
    /// Pass an empty string to remove previously-defined discounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<PostInvoicesInvoiceParamsDiscounts>>,

    /// The date on which payment for this invoice is due.
    ///
    /// Only valid for invoices where `collection_method=send_invoice`.
    /// This field can only be updated on `draft` invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub due_date: Option<crate::params::Timestamp>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Footer to be displayed on the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account (if any) for which the funds of the invoice payment are intended.
    ///
    /// If set, the invoice will be presented with the branding and support information of the specified account.
    /// See the [Invoices with Connect](https://stripe.com/docs/billing/invoices/connect) documentation for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_settings: Option<PostInvoicesInvoiceParamsPaymentSettings>,

    /// Options for invoice PDF rendering.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<PostInvoicesInvoiceParamsRenderingOptions>,

    /// Extra information about a charge for the customer's credit card statement.
    ///
    /// It must contain at least one letter.
    /// If not specified and this invoice is part of a subscription, the default `statement_descriptor` will be set to the first subscription item's product's `statement_descriptor`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
    ///
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostInvoicesInvoiceParamsTransferData>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoicePayParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// In cases where the source used to pay the invoice has insufficient funds, passing `forgive=true` controls whether a charge should be attempted for the full amount available on the source, up to the amount to fully pay the invoice.
    ///
    /// This effectively forgives the difference between the amount available on the source and the amount due.
    /// Passing `forgive=false` will fail the charge if the source hasn't been pre-funded with the right amount.
    /// An example for this case is with ACH Credit Transfers and wires: if the amount wired is less than the amount due by a small amount, you might want to forgive the difference.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub forgive: Option<bool>,

    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the payment_method param or the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<String>,

    /// Indicates if a customer is on or off-session while an invoice payment is attempted.
    ///
    /// Defaults to `true` (off-session).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_session: Option<crate::resources::PaymentIntentOffSession>,

    /// Boolean representing whether an invoice is paid outside of Stripe.
    ///
    /// This will result in no charge being made.
    /// Defaults to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paid_out_of_band: Option<bool>,

    /// A PaymentMethod to be charged.
    ///
    /// The PaymentMethod must be the ID of a PaymentMethod belonging to the customer associated with the invoice being paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// A payment source to be charged.
    ///
    /// The source must be the ID of a source belonging to the customer associated with the invoice being paid.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceFinalizeParams {
    /// Controls whether Stripe will perform [automatic collection](https://stripe.com/docs/invoicing/automatic-charging) of the invoice.
    ///
    /// When `false`, the invoice's state will not automatically advance without an explicit action.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub auto_advance: Option<bool>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<GetInvoicesUpcomingLinesParamsAutomaticTax>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_details: Option<GetInvoicesUpcomingLinesParamsCustomerDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<GetInvoicesUpcomingLinesParamsDiscounts>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_items: Option<Vec<GetInvoicesUpcomingLinesParamsInvoiceItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub schedule: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_billing_cycle_anchor:
        Option<GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchor>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_at_period_end: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel_now: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_default_tax_rates: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_items: Option<Vec<GetInvoicesUpcomingLinesParamsSubscriptionItems>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_behavior:
        Option<GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_proration_date: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_start_date: Option<crate::params::Timestamp>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_end: Option<GetInvoicesUpcomingLinesParamsSubscriptionTrialEnd>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_trial_from_plan: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceSendParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceMarkUncollectibleParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceVoidParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceBillingReason {
    AutomaticPendingInvoiceItemInvoice,
    Manual,
    QuoteAccept,
    Subscription,
    SubscriptionCreate,
    SubscriptionCycle,
    SubscriptionThreshold,
    SubscriptionUpdate,
    Upcoming,
}

impl InvoiceBillingReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AutomaticPendingInvoiceItemInvoice => "automatic_pending_invoice_item_invoice",
            Self::Manual => "manual",
            Self::QuoteAccept => "quote_accept",
            Self::Subscription => "subscription",
            Self::SubscriptionCreate => "subscription_create",
            Self::SubscriptionCycle => "subscription_cycle",
            Self::SubscriptionThreshold => "subscription_threshold",
            Self::SubscriptionUpdate => "subscription_update",
            Self::Upcoming => "upcoming",
        }
    }
}

impl AsRef<str> for InvoiceBillingReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceBillingReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InvoiceBillingReason {
    fn default() -> Self {
        Self::AutomaticPendingInvoiceItemInvoice
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl InvoiceCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for InvoiceCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InvoiceCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceCustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl InvoiceCustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for InvoiceCustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceCustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InvoiceCustomerTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoiceStatus {
    Deleted,
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl InvoiceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Deleted => "deleted",
            Self::Draft => "draft",
            Self::Open => "open",
            Self::Paid => "paid",
            Self::Uncollectible => "uncollectible",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for InvoiceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InvoiceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InvoiceStatus {
    fn default() -> Self {
        Self::Deleted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnedGetInvoicesSearchObject {
    SearchResult,
}

impl ReturnedGetInvoicesSearchObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for ReturnedGetInvoicesSearchObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnedGetInvoicesSearchObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ReturnedGetInvoicesSearchObject {
    fn default() -> Self {
        Self::SearchResult
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl GetInvoicesParamsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for GetInvoicesParamsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesParamsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesParamsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesParamsStatus {
    Draft,
    Open,
    Paid,
    Uncollectible,
    Void,
}

impl GetInvoicesParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Open => "open",
            Self::Paid => "paid",
            Self::Uncollectible => "uncollectible",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for GetInvoicesParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesParamsStatus {
    fn default() -> Self {
        Self::Draft
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetails {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<GetInvoicesUpcomingParamsCustomerDetailsAddress>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<GetInvoicesUpcomingParamsCustomerDetailsShipping>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<GetInvoicesUpcomingParamsCustomerDetailsTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<GetInvoicesUpcomingParamsCustomerDetailsTaxExempt>,

    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<GetInvoicesUpcomingParamsCustomerDetailsTaxIds>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsInvoiceItems {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Explicitly controls whether discounts apply to this invoice item.
    ///
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<GetInvoicesUpcomingParamsInvoiceItemsDiscounts>>,

    /// The ID of the invoice item to update in preview.
    ///
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,

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
    pub period: Option<GetInvoicesUpcomingParamsInvoiceItemsPeriod>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<GetInvoicesUpcomingParamsInvoiceItemsPriceData>,

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
    pub tax_behavior: Option<GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates that apply to the item.
    ///
    /// When set, any `default_tax_rates` do not apply to this item.
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchor {
    NowOrUnchanged(GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged),
    Timestamp(crate::params::Timestamp),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<GetInvoicesUpcomingParamsSubscriptionItemsBillingThresholds>,

    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,

    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<GetInvoicesUpcomingParamsSubscriptionItemsPriceData>,

    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl GetInvoicesUpcomingParamsSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsSubscriptionProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionTrialEnd {
    Now(GetInvoicesUpcomingParamsSubscriptionTrialEndNow),
    Timestamp(crate::params::Timestamp),
}

/// Settings for automatic tax lookup for this invoice.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostInvoicesParamsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// Revise an existing invoice.
///
/// The new invoice will be created in `status=draft`.
/// See the [revision documentation](https://stripe.com/docs/invoicing/invoice-revisions) for more details.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsFromInvoice {
    /// The relation between the new invoice and the original invoice.
    ///
    /// Currently, only 'revision' is permitted.
    pub action: PostInvoicesParamsFromInvoiceAction,

    /// The `id` of the invoice that will be cloned.
    pub invoice: String,
}

/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettings {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,

    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types: Option<Vec<PostInvoicesParamsPaymentSettingsPaymentMethodTypes>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPendingInvoiceItemsBehavior {
    Exclude,
    Include,
    IncludeAndRequire,
}

impl PostInvoicesParamsPendingInvoiceItemsBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclude => "exclude",
            Self::Include => "include",
            Self::IncludeAndRequire => "include_and_require",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsPendingInvoiceItemsBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsPendingInvoiceItemsBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPendingInvoiceItemsBehavior {
    fn default() -> Self {
        Self::Exclude
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<PostInvoicesParamsRenderingOptionsAmountTaxDisplay>,
}

/// If specified, the funds from the invoice will be transferred to the destination and the ID of the resulting transfer will be found on the invoice's charge.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

/// Settings for automatic tax lookup for this invoice.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostInvoicesInvoiceParamsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostInvoicesInvoiceParamsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesInvoiceParamsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesInvoiceParamsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsCustomFields {
    /// The name of the custom field.
    ///
    /// This may be up to 30 characters.
    pub name: String,

    /// The value of the custom field.
    ///
    /// This may be up to 30 characters.
    pub value: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

/// Configuration settings for the PaymentIntent that is generated when the invoice is finalized.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettings {
    /// ID of the mandate to be used for this invoice.
    ///
    /// It must correspond to the payment method used to pay the invoice, including the invoice's default_payment_method or default_source, if set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_mandate: Option<String>,

    /// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_options:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptions>,

    /// The list of payment method types (e.g.
    ///
    /// card) to provide to the invoice’s PaymentIntent.
    /// If not set, Stripe attempts to automatically determine the types to use by looking at the invoice’s default payment method, the subscription’s default payment method, the customer’s default payment method, and your [invoice template settings](https://dashboard.stripe.com/settings/billing/invoice).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_types:
        Option<Vec<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display: Option<PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsTransferData {
    /// The amount that will be transferred automatically when the invoice is paid.
    ///
    /// If no amount is set, the full amount is transferred.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsAutomaticTax {
    /// Whether Stripe automatically computes tax on this invoice.
    ///
    /// Note that incompatible invoice items (invoice items with manually specified [tax rates](https://stripe.com/docs/api/tax_rates), negative amounts, or `tax_behavior=unspecified`) cannot be added to automatic tax invoices.
    pub enabled: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetails {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<GetInvoicesUpcomingLinesParamsCustomerDetailsAddress>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<GetInvoicesUpcomingLinesParamsCustomerDetailsShipping>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<GetInvoicesUpcomingLinesParamsCustomerDetailsTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt>,

    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_ids: Option<Vec<GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIds>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsDiscounts {
    /// ID of the coupon to create a new discount for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// ID of an existing discount on the object (or one of its ancestors) to reuse.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsInvoiceItems {
    /// The integer amount in cents (or local equivalent) of previewed invoice item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Only applicable to new invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// An arbitrary string which you can attach to the invoice item.
    ///
    /// The description is displayed in the invoice for easy tracking.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Explicitly controls whether discounts apply to this invoice item.
    ///
    /// Defaults to true, except for negative invoice items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discountable: Option<bool>,

    /// The coupons to redeem into discounts for the invoice item in the preview.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discounts: Option<Vec<GetInvoicesUpcomingLinesParamsInvoiceItemsDiscounts>>,

    /// The ID of the invoice item to update in preview.
    ///
    /// If not specified, a new invoice item will be added to the preview of the upcoming invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoiceitem: Option<String>,

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
    pub period: Option<GetInvoicesUpcomingLinesParamsInvoiceItemsPeriod>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<GetInvoicesUpcomingLinesParamsInvoiceItemsPriceData>,

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
    pub tax_behavior: Option<GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior>,

    /// A [tax code](https://stripe.com/docs/tax/tax-categories) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_code: Option<String>,

    /// The tax rates that apply to the item.
    ///
    /// When set, any `default_tax_rates` do not apply to this item.
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchor {
    NowOrUnchanged(GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged),
    Timestamp(crate::params::Timestamp),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsSubscriptionItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds:
        Option<GetInvoicesUpcomingLinesParamsSubscriptionItemsBillingThresholds>,

    /// Delete all usage for a given subscription item.
    ///
    /// Allowed only when `deleted` is set to `true` and the current plan's `usage_type` is `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub clear_usage: Option<bool>,

    /// A flag that, if set to `true`, will delete the specified item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deleted: Option<bool>,

    /// Subscription item to update.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Plan ID for this item, as a string.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    ///
    /// When changing a subscription item's price, `quantity` is set to 1 unless a `quantity` parameter is provided.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceData>,

    /// Quantity for this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsSubscriptionProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionTrialEnd {
    Now(GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow),
    Timestamp(crate::params::Timestamp),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetailsAddress {
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

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetailsShipping {
    /// Customer shipping address.
    pub address: GetInvoicesUpcomingParamsCustomerDetailsShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Tax details about the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetailsTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl GetInvoicesUpcomingParamsCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsCustomerDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType,

    /// Value of the tax ID.
    pub value: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsInvoiceItemsDiscounts {
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
pub struct GetInvoicesUpcomingParamsInvoiceItemsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::params::Timestamp,

    /// The start of the period.
    pub start: crate::params::Timestamp,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsInvoiceItemsPriceData {
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
    pub tax_behavior: Option<GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior>,

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
pub enum GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsInvoiceItemsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    Now,
    Unchanged,
}

impl GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
            Self::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    fn default() -> Self {
        Self::Now
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsSubscriptionItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsSubscriptionItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurring,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior>,

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
pub enum GetInvoicesUpcomingParamsSubscriptionTrialEndNow {
    Now,
}

impl GetInvoicesUpcomingParamsSubscriptionTrialEndNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsSubscriptionTrialEndNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsSubscriptionTrialEndNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsSubscriptionTrialEndNow {
    fn default() -> Self {
        Self::Now
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsFromInvoiceAction {
    Revision,
}

impl PostInvoicesParamsFromInvoiceAction {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Revision => "revision",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsFromInvoiceAction {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsFromInvoiceAction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsFromInvoiceAction {
    fn default() -> Self {
        Self::Revision
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsKonbini>,

    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Paynow => "paynow",
            Self::Promptpay => "promptpay",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl PostInvoicesParamsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}
/// Payment-method-specific configuration to provide to the invoice’s PaymentIntent.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptions {
    /// If paying by `acss_debit`, this sub-hash contains details about the Canadian pre-authorized debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit>,

    /// If paying by `bancontact`, this sub-hash contains details about the Bancontact payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact>,

    /// If paying by `card`, this sub-hash contains details about the Card payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCard>,

    /// If paying by `customer_balance`, this sub-hash contains details about the Bank transfer payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance>,

    /// If paying by `konbini`, this sub-hash contains details about the Konbini payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini>,

    /// If paying by `us_bank_account`, this sub-hash contains details about the ACH direct debit payment method options to pass to the invoice’s PaymentIntent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Boleto,
    Card,
    CustomerBalance,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Konbini,
    Link,
    Paynow,
    Promptpay,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Paynow => "paynow",
            Self::Promptpay => "promptpay",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesInvoiceParamsRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetailsAddress {
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

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetailsShipping {
    /// Customer shipping address.
    pub address: GetInvoicesUpcomingLinesParamsCustomerDetailsShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Tax details about the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetailsTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIds {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType,

    /// Value of the tax ID.
    pub value: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsInvoiceItemsDiscounts {
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
pub struct GetInvoicesUpcomingLinesParamsInvoiceItemsPeriod {
    /// The end of the period, which must be greater than or equal to the start.
    pub end: crate::params::Timestamp,

    /// The start of the period.
    pub start: crate::params::Timestamp,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsInvoiceItemsPriceData {
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
    pub tax_behavior: Option<GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior>,

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
pub enum GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsInvoiceItemsTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    Now,
    Unchanged,
}

impl GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
            Self::Unchanged => "unchanged",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsSubscriptionBillingCycleAnchorNowOrUnchanged {
    fn default() -> Self {
        Self::Now
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsSubscriptionItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurring,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior>,

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
pub enum GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow {
    Now,
}

impl GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsSubscriptionTrialEndNow {
    fn default() -> Self {
        Self::Now
    }
}
/// Customer shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsCustomerDetailsShippingAddress {
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

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType {
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

impl GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType {
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

impl AsRef<str> for GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsCustomerDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsInvoiceItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallments>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer:
        Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer>,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsKonbini {}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections: Option<
        PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections,
    >,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebit {
    /// Additional fields for Mandate creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate_options:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions>,

    /// Verification method for the intent.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verification_method: Option<
        PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontact {
    /// Preferred language of the Bancontact authorization page that the customer is redirected to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_language: Option<
        PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCard {
    /// Installment configuration for payments attempted on this invoice (Mexico Only).
    ///
    /// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub installments:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments>,

    /// We strongly recommend that you rely on our SCA Engine to automatically prompt your customers for authentication based on risk level and [other requirements](https://stripe.com/docs/strong-customer-authentication).
    ///
    /// However, if you wish to request 3D Secure based on logic from your own fraud engine, provide this option.
    /// Read our guide on [manually requesting 3D Secure](https://stripe.com/docs/payments/3d-secure#manual-three-ds) for more information on how this configuration interacts with Radar and our SCA Engine.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub request_three_d_secure:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalance {
    /// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank_transfer: Option<
        PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer,
    >,

    /// The funding method type to be used when there are not enough funds in the customer balance.
    ///
    /// Permitted values include: `bank_transfer`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub funding_type: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsKonbini {}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccount {
    /// Additional fields for Financial Connections Session creation.
#[serde(skip_serializing_if = "Option::is_none")]
pub financial_connections: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections>,

    /// Verification method for the intent.
#[serde(skip_serializing_if = "Option::is_none")]
pub verification_method: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod>,
}

/// Customer shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsCustomerDetailsShippingAddress {
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

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType {
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

impl GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType {
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

impl AsRef<str> for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsCustomerDetailsTaxIdsType {
    fn default() -> Self {
        Self::AeTrn
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsInvoiceItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingParamsSubscriptionItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction_type: Option<
        PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
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
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    fn default() -> Self {
        Self::De
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this invoice.
    /// Setting to false will prevent any selected plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The selected installment plan to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    fn default() -> Self {
        Self::Any
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
pub type_: Option<String>,
}

/// Additional fields for Financial Connections Session creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<Vec<PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}
/// Additional fields for Mandate creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptions {
    /// Transaction type of the mandate.
#[serde(skip_serializing_if = "Option::is_none")]
pub transaction_type: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    Automatic,
    Instant,
    Microdeposits,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
    De,
    En,
    Fr,
    Nl,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage {
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
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsBancontactPreferredLanguage
{
    fn default() -> Self {
        Self::De
    }
}
/// Installment configuration for payments attempted on this invoice (Mexico Only).
///
/// For more information, see the [installments integration guide](https://stripe.com/docs/payments/installments).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallments {
    /// Setting to true enables installments for this invoice.
    /// Setting to false will prevent any selected plan from applying to a payment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,

    /// The selected installment plan to use for this invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan:
        Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    Any,
    Automatic,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Any => "any",
            Self::Automatic => "automatic",
        }
    }
}

impl AsRef<str>
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardRequestThreeDSecure
{
    fn default() -> Self {
        Self::Any
    }
}
/// Configuration for the bank transfer funding type, if the `funding_type` is set to `bank_transfer`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
#[serde(skip_serializing_if = "Option::is_none")]
pub eu_bank_transfer: Option<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer>,

    /// The bank transfer type that can be used for funding.
    ///
    /// Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
#[serde(rename = "type")]
#[serde(skip_serializing_if = "Option::is_none")]
pub type_: Option<String>,
}

/// Additional fields for Financial Connections Session creation.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnections {
    /// The list of permissions to request.
    ///
    /// If this parameter is passed, the `payment_method` permission must be included.
    /// Valid permissions include: `balances`, `ownership`, `payment_method`, and `transactions`.
#[serde(skip_serializing_if = "Option::is_none")]
pub permissions: Option<Vec<PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    Automatic,
    Instant,
    Microdeposits,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Instant => "instant",
            Self::Microdeposits => "microdeposits",
        }
    }
}

impl AsRef<str>
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountVerificationMethod
{
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetInvoicesUpcomingLinesParamsSubscriptionItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Business => "business",
            Self::Personal => "personal",
        }
    }
}

impl AsRef<str>
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    fn default() -> Self {
        Self::Business
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,

    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval: PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,

    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}

/// Configuration for eu_bank_transfer funding type.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Balances => "balances",
Self::Ownership => "ownership",
Self::PaymentMethod => "payment_method",
Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType
{
    Business,
    Personal,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Business => "business",
Self::Personal => "personal",
        }
    }
}

impl AsRef<str> for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsAcssDebitMandateOptionsTransactionType {
    fn default() -> Self {
        Self::Business
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlan {
    /// For `fixed_count` installment plans, this is the number of installment payments your customer will make to their credit card.
    pub count: u64,

    /// For `fixed_count` installment plans, this is the interval between installment payments your customer will make to their credit card.
    /// One of `month`.
    pub interval:
        PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval,

    /// Type of installment plan, one of `fixed_count`.
    #[serde(rename = "type")]
    pub type_: PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType,
}

/// Configuration for eu_bank_transfer funding type.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer
{
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions
{
    Balances,
    Ownership,
    PaymentMethod,
    Transactions,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Balances => "balances",
Self::Ownership => "ownership",
Self::PaymentMethod => "payment_method",
Self::Transactions => "transactions",
        }
    }
}

impl AsRef<str> for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsUsBankAccountFinancialConnectionsPermissions {
    fn default() -> Self {
        Self::Balances
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl AsRef<str>
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    fn default() -> Self {
        Self::Month
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str> for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostInvoicesParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    fn default() -> Self {
        Self::FixedCount
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    Month,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Month => "month",
        }
    }
}

impl AsRef<str>
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanInterval
{
    fn default() -> Self {
        Self::Month
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    FixedCount,
}

impl PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FixedCount => "fixed_count",
        }
    }
}

impl AsRef<str>
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostInvoicesInvoiceParamsPaymentSettingsPaymentMethodOptionsCardInstallmentsPlanType
{
    fn default() -> Self {
        Self::FixedCount
    }
}
pub fn get_invoices_search(
    client: &crate::Client,
    params: GetInvoicesSearchParams,
) -> crate::Response<ReturnedGetInvoicesSearch> {
    client.get_query("/invoices/search", params)
}

pub fn get_invoices(
    client: &crate::Client,
    params: GetInvoicesParams,
) -> crate::Response<crate::params::List<crate::generated::Invoice>> {
    client.get_query("/invoices", params)
}

pub fn get_invoices_upcoming(
    client: &crate::Client,
    params: GetInvoicesUpcomingParams,
) -> crate::Response<crate::generated::Invoice> {
    client.get_query("/invoices/upcoming", params)
}

pub fn post_invoices(
    client: &crate::Client,
    params: PostInvoicesParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form("/invoices", params)
}

pub fn get_invoices_invoice(
    client: &crate::Client,
    invoice: String,
    params: GetInvoicesInvoiceParams,
) -> crate::Response<crate::generated::Invoice> {
    client.get_query(&format!("/invoices/{invoice}", invoice = invoice), params)
}

pub fn post_invoices_invoice(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoiceParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}", invoice = invoice), params)
}

pub fn delete_invoices_invoice(
    client: &crate::Client,
    invoice: String,
) -> crate::Response<crate::generated::DeletedInvoice> {
    client.delete(&format!("/invoices/{invoice}", invoice = invoice))
}

pub fn post_invoices_invoice_pay(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoicePayParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}/pay", invoice = invoice), params)
}

pub fn post_invoices_invoice_finalize(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoiceFinalizeParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}/finalize", invoice = invoice), params)
}

pub fn get_invoices_upcoming_lines(
    client: &crate::Client,
    params: GetInvoicesUpcomingLinesParams,
) -> crate::Response<crate::params::List<crate::generated::LineItem>> {
    client.get_query("/invoices/upcoming/lines", params)
}

pub fn post_invoices_invoice_send(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoiceSendParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}/send", invoice = invoice), params)
}

pub fn post_invoices_invoice_mark_uncollectible(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoiceMarkUncollectibleParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}/mark_uncollectible", invoice = invoice), params)
}

pub fn post_invoices_invoice_void(
    client: &crate::Client,
    invoice: String,
    params: PostInvoicesInvoiceVoidParams,
) -> crate::Response<crate::generated::Invoice> {
    client.post_form(&format!("/invoices/{invoice}/void", invoice = invoice), params)
}
