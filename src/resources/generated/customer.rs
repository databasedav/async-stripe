/// This object represents a customer of your business.
///
/// It lets you create recurring charges and track payments that belong to the same customer.  Related guide: [Save a card during payment](https://stripe.com/docs/payments/save-during-payment).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Customer {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<crate::generated::Address>,

    /// Current balance, if any, being stored on the customer.
    ///
    /// If negative, the customer has credit to apply to their next invoice.
    /// If positive, the customer has an amount owed that will be added to their next invoice.
    /// The balance does not refer to any unpaid invoices; it solely takes into account amounts that have yet to be successfully applied to any invoice.
    /// This balance is only taken into account as invoices are finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// The current funds being held by Stripe on behalf of the customer.
    ///
    /// These funds can be applied towards payment intents with source "cash_balance".
    /// The settings[reconciliation_mode] field describes whether these funds are applied to such payment intents manually or automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<crate::generated::CashBalance>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) the customer can be charged in for recurring billing purposes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// ID of the default payment source for the customer.
    ///
    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/object#customer_object-invoice_settings-default_payment_method) field instead.
    pub default_source: Option<Vec<crate::generated::PaymentSource>>,

    /// When the customer's latest invoice is billed by charging automatically, `delinquent` is `true` if the invoice's latest charge failed.
    ///
    /// When the customer's latest invoice is billed by sending an invoice, `delinquent` is `true` if the invoice isn't paid by its due date.  If an invoice is marked uncollectible by [dunning](https://stripe.com/docs/billing/automatic-collection), `delinquent` doesn't get reset to `false`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub delinquent: Option<bool>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// Describes the current discount active on the customer, if there is one.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub discount: Option<crate::generated::Discount>,

    /// The customer's email address.
    pub email: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// The current multi-currency balances, if any, being stored on the customer.
    ///
    /// If positive in a currency, the customer has a credit to apply to their next invoice denominated in that currency.
    /// If negative, the customer has an amount owed that will be added to their next invoice denominated in that currency.
    /// These balances do not refer to any unpaid invoices.
    /// They solely track amounts that have yet to be successfully applied to any invoice.
    /// A balance in a particular currency is only applied to any invoice as an invoice in that currency is finalized.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_credit_balance: Option<i64>,

    /// The prefix for the customer used to generate unique invoice numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<crate::generated::InvoiceSettingCustomerSetting>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The suffix of the customer's next invoice number, e.g., 0001.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The customer's preferred locales (languages), ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// Mailing and shipping address for the customer.
    ///
    /// Appears on invoices emailed to this customer.
    pub shipping: Option<crate::generated::Shipping>,

    /// The customer's payment sources, if any.
    #[serde(default)]
    pub sources: crate::params::List<crate::generated::PaymentSource>,

    /// The customer's current subscriptions, if any.
    #[serde(default)]
    pub subscriptions: crate::params::List<crate::generated::Subscription>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<crate::generated::CustomerTax>,

    /// Describes the customer's tax exemption status.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    /// When set to `reverse`, invoice and receipt PDFs include the text **"Reverse charge"**.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<CustomerTaxExempt>,

    /// The customer's tax IDs.
    #[serde(default)]
    pub tax_ids: crate::params::List<crate::generated::TaxId>,

    /// ID of the test clock this customer belongs to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<Vec<crate::generated::TestHelpersTestClock>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ReturnedGetCustomersSearch {
    pub data: Vec<crate::generated::Customer>,

    pub has_more: bool,

    pub next_page: Option<String>,

    /// String representing the object's type.
    ///
    /// Objects of the same type share the same value.
    pub object: ReturnedGetCustomersSearchObject,

    /// The total number of objects that match the query, only accurate up to 10,000.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_count: Option<u64>,

    pub url: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersSearchParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub page: Option<String>,

    pub query: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParams {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostCustomersParamsAddress>,

    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<PostCustomersParamsCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostCustomersParamsInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostCustomersParamsShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<PostCustomersParamsTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PostCustomersParamsTaxExempt>,

    /// The customer's tax IDs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_id_data: Option<Vec<PostCustomersParamsTaxIdData>>,

    /// ID of the test clock to attach to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub test_clock: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedGetCustomersCustomer {
    Customer(crate::generated::Customer),
    DeletedCustomer(crate::generated::DeletedCustomer),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParams {
    /// The customer's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostCustomersCustomerParamsAddress>,

    /// An integer amount in cents (or local equivalent) that represents the customer's current balance, which affect the customer's future invoices.
    ///
    /// A negative amount represents a credit that decreases the amount due on an invoice; a positive amount increases the amount due on an invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<i64>,

    /// Balance information and default balance settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash_balance: Option<PostCustomersCustomerParamsCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// If you are using payment methods created via the PaymentMethods API, see the [invoice_settings.default_payment_method](https://stripe.com/docs/api/customers/update#update_customer-invoice_settings-default_payment_method) parameter.
    ///
    /// Provide the ID of a payment source already attached to this customer to make it this customer's default payment source.
    ///
    /// If you want to add a new payment source and make it the default, see the [source](https://stripe.com/docs/api/customers/update#update_customer-source) property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_source: Option<String>,

    /// An arbitrary string that you can attach to a customer object.
    ///
    /// It is displayed alongside the customer in the dashboard.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// Customer's email address.
    ///
    /// It's displayed alongside the customer in your dashboard and can be useful for searching and tracking.
    /// This may be up to *512 characters*.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The prefix for the customer used to generate unique invoice numbers.
    ///
    /// Must be 3–12 uppercase letters or numbers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_prefix: Option<String>,

    /// Default invoice settings for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostCustomersCustomerParamsInvoiceSettings>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The customer's full name or business name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// The sequence to be used on the customer's next invoice.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub next_invoice_sequence: Option<i64>,

    /// The customer's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// Customer's preferred languages, ordered by preference.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_locales: Option<Vec<String>>,

    /// The API ID of a promotion code to apply to the customer.
    ///
    /// The customer will have a discount applied on all recurring payments.
    /// Charges you create through the API will not have the discount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promotion_code: Option<String>,

    /// The customer's shipping information.
    ///
    /// Appears on invoices emailed to this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostCustomersCustomerParamsShipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,

    /// Tax details about the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax: Option<PostCustomersCustomerParamsTax>,

    /// The customer's tax exemption.
    ///
    /// One of `none`, `exempt`, or `reverse`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_exempt: Option<PostCustomersCustomerParamsTaxExempt>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub validate: Option<bool>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerPaymentMethodsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(rename = "type")]
    pub type_: GetCustomersCustomerPaymentMethodsParamsType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerPaymentMethodsPaymentMethodParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerBalanceTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersCustomersCustomerFundCashBalanceParams {
    /// Amount to be used for this test cash balance transaction.
    ///
    /// A positive integer representing how much to fund in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal) (e.g., 100 cents to fund $1.00 or 100 to fund ¥100, a zero-decimal currency).
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A description of the test funding.
    ///
    /// This simulates free-text references supplied by customers when making bank transfers to their cash balance.
    /// You can use this to test how Stripe's [reconciliation algorithm](https://stripe.com/docs/payments/customer-balance/reconciliation) applies to different user inputs.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reference: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerFundingInstructionsParams {
    /// Additional parameters for `bank_transfer` funding types.
    pub bank_transfer: PostCustomersCustomerFundingInstructionsParamsBankTransfer,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The `funding_type` to get the instructions for.
    pub funding_type: PostCustomersCustomerFundingInstructionsParamsFundingType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl CustomerTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for CustomerTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReturnedGetCustomersSearchObject {
    SearchResult,
}

impl ReturnedGetCustomersSearchObject {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::SearchResult => "search_result",
        }
    }
}

impl AsRef<str> for ReturnedGetCustomersSearchObject {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReturnedGetCustomersSearchObject {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ReturnedGetCustomersSearchObject {
    fn default() -> Self {
        Self::SearchResult
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsAddress {
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

/// Balance information and default balance settings for this customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PostCustomersParamsCashBalanceSettings>,
}

/// Default invoice settings for this customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsInvoiceSettings {
    /// Default custom fields to be displayed on invoices for this customer.
    ///
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<PostCustomersParamsInvoiceSettingsCustomFields>>,

    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<PostCustomersParamsInvoiceSettingsRenderingOptions>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsShipping {
    /// Customer shipping address.
    pub address: PostCustomersParamsShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Tax details about the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersParamsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PostCustomersParamsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PostCustomersParamsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersParamsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersParamsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsTaxIdData {
    /// Type of the tax ID, one of `ae_trn`, `au_abn`, `au_arn`, `bg_uic`, `br_cnpj`, `br_cpf`, `ca_bn`, `ca_gst_hst`, `ca_pst_bc`, `ca_pst_mb`, `ca_pst_sk`, `ca_qst`, `ch_vat`, `cl_tin`, `eg_tin`, `es_cif`, `eu_oss_vat`, `eu_vat`, `gb_vat`, `ge_vat`, `hk_br`, `hu_tin`, `id_npwp`, `il_vat`, `in_gst`, `is_vat`, `jp_cn`, `jp_rn`, `jp_trn`, `ke_pin`, `kr_brn`, `li_uid`, `mx_rfc`, `my_frp`, `my_itn`, `my_sst`, `no_vat`, `nz_gst`, `ph_tin`, `ru_inn`, `ru_kpp`, `sa_vat`, `sg_gst`, `sg_uen`, `si_tin`, `th_vat`, `tr_tin`, `tw_vat`, `ua_vat`, `us_ein`, or `za_vat`.
    #[serde(rename = "type")]
    pub type_: PostCustomersParamsTaxIdDataType,

    /// Value of the tax ID.
    pub value: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsAddress {
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

/// Balance information and default balance settings for this customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsCashBalance {
    /// Settings controlling the behavior of the customer's cash balance,
    /// such as reconciliation of funds received.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PostCustomersCustomerParamsCashBalanceSettings>,
}

/// Default invoice settings for this customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsInvoiceSettings {
    /// Default custom fields to be displayed on invoices for this customer.
    ///
    /// When updating, pass an empty string to remove previously-defined fields.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub custom_fields: Option<Vec<PostCustomersCustomerParamsInvoiceSettingsCustomFields>>,

    /// ID of a payment method that's attached to the customer, to be used as the customer's default payment method for subscriptions and invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// Default footer to be displayed on invoices for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub footer: Option<String>,

    /// Default options for invoice PDF rendering for this customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rendering_options: Option<PostCustomersCustomerParamsInvoiceSettingsRenderingOptions>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsShipping {
    /// Customer shipping address.
    pub address: PostCustomersCustomerParamsShippingAddress,

    /// Customer name.
    pub name: String,

    /// Customer phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Tax details about the customer.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsTax {
    /// A recent IP address of the customer used for tax reporting and tax location inference.
    ///
    /// Stripe recommends updating the IP address when a new PaymentMethod is attached or the address field on the customer is updated.
    /// We recommend against updating this field more frequently since it could result in unexpected tax location/reporting outcomes.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip_address: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerParamsTaxExempt {
    Exempt,
    None,
    Reverse,
}

impl PostCustomersCustomerParamsTaxExempt {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exempt => "exempt",
            Self::None => "none",
            Self::Reverse => "reverse",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerParamsTaxExempt {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerParamsTaxExempt {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerParamsTaxExempt {
    fn default() -> Self {
        Self::Exempt
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetCustomersCustomerPaymentMethodsParamsType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl GetCustomersCustomerPaymentMethodsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for GetCustomersCustomerPaymentMethodsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetCustomersCustomerPaymentMethodsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetCustomersCustomerPaymentMethodsParamsType {
    fn default() -> Self {
        Self::AcssDebit
    }
}
/// Additional parameters for `bank_transfer` funding types.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerFundingInstructionsParamsBankTransfer {
    /// Configuration for eu_bank_transfer funding type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer:
        Option<PostCustomersCustomerFundingInstructionsParamsBankTransferEuBankTransfer>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types: Option<
        Vec<PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes>,
    >,

    /// The type of the `bank_transfer`.
    #[serde(rename = "type")]
    pub type_: PostCustomersCustomerFundingInstructionsParamsBankTransferType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerFundingInstructionsParamsFundingType {
    BankTransfer,
}

impl PostCustomersCustomerFundingInstructionsParamsFundingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankTransfer => "bank_transfer",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerFundingInstructionsParamsFundingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerFundingInstructionsParamsFundingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerFundingInstructionsParamsFundingType {
    fn default() -> Self {
        Self::BankTransfer
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode: Option<PostCustomersParamsCashBalanceSettingsReconciliationMode>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsInvoiceSettingsCustomFields {
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
pub struct PostCustomersParamsInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}

/// Customer shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersParamsShippingAddress {
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
pub enum PostCustomersParamsTaxIdDataType {
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

impl PostCustomersParamsTaxIdDataType {
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

impl AsRef<str> for PostCustomersParamsTaxIdDataType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersParamsTaxIdDataType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersParamsTaxIdDataType {
    fn default() -> Self {
        Self::AeTrn
    }
}
/// Settings controlling the behavior of the customer's cash balance,
/// such as reconciliation of funds received.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsCashBalanceSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode:
        Option<PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsInvoiceSettingsCustomFields {
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
pub struct PostCustomersCustomerParamsInvoiceSettingsRenderingOptions {
    /// How line-item prices and amounts will be displayed with respect to tax on invoice PDFs.
    ///
    /// One of `exclude_tax` or `include_inclusive_tax`.
    /// `include_inclusive_tax` will include inclusive tax (and exclude exclusive tax) in invoice PDF amounts.
    /// `exclude_tax` will exclude all tax (inclusive and exclusive alike) from invoice PDF amounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_tax_display:
        Option<PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay>,
}

/// Customer shipping address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerParamsShippingAddress {
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

/// Configuration for eu_bank_transfer funding type.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerFundingInstructionsParamsBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str>
    for PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerFundingInstructionsParamsBankTransferRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerFundingInstructionsParamsBankTransferType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl PostCustomersCustomerFundingInstructionsParamsBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerFundingInstructionsParamsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerFundingInstructionsParamsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerFundingInstructionsParamsBankTransferType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersParamsCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl PostCustomersParamsCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostCustomersParamsCashBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersParamsCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersParamsCashBalanceSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerParamsCashBalanceSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    ExcludeTax,
    IncludeInclusiveTax,
}

impl PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ExcludeTax => "exclude_tax",
            Self::IncludeInclusiveTax => "include_inclusive_tax",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerParamsInvoiceSettingsRenderingOptionsAmountTaxDisplay {
    fn default() -> Self {
        Self::ExcludeTax
    }
}
pub fn get_customers_search(
    client: &crate::Client,
    params: GetCustomersSearchParams,
) -> crate::Response<ReturnedGetCustomersSearch> {
    client.get_query("/customers/search", params)
}

pub fn get_customers(
    client: &crate::Client,
    params: GetCustomersParams,
) -> crate::Response<crate::params::List<crate::generated::Customer>> {
    client.get_query("/customers", params)
}

pub fn post_customers(
    client: &crate::Client,
    params: PostCustomersParams,
) -> crate::Response<crate::generated::Customer> {
    client.post_form("/customers", params)
}

pub fn get_customers_customer(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerParams,
) -> crate::Response<ReturnedGetCustomersCustomer> {
    client.get_query(&format!("/customers/{customer}", customer = customer), params)
}

pub fn post_customers_customer(
    client: &crate::Client,
    customer: String,
    params: PostCustomersCustomerParams,
) -> crate::Response<crate::generated::Customer> {
    client.post_form(&format!("/customers/{customer}", customer = customer), params)
}

pub fn delete_customers_customer(
    client: &crate::Client,
    customer: String,
) -> crate::Response<crate::generated::DeletedCustomer> {
    client.delete(&format!("/customers/{customer}", customer = customer))
}

pub fn get_customers_customer_payment_methods(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerPaymentMethodsParams,
) -> crate::Response<crate::params::List<crate::generated::PaymentMethod>> {
    client.get_query(&format!("/customers/{customer}/payment_methods", customer = customer), params)
}

pub fn get_customers_customer_payment_methods_payment_method(
    client: &crate::Client,
    customer: String,
    payment_method: String,
    params: GetCustomersCustomerPaymentMethodsPaymentMethodParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.get_query(
        &format!(
            "/customers/{customer}/payment_methods/{payment_method}",
            customer = customer,
            payment_method = payment_method
        ),
        params,
    )
}

pub fn get_customers_customer_balance_transactions(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerBalanceTransactionsParams,
) -> crate::Response<crate::params::List<crate::generated::CustomerBalanceTransaction>> {
    client.get_query(
        &format!("/customers/{customer}/balance_transactions", customer = customer),
        params,
    )
}

pub fn post_test_helpers_customers_customer_fund_cash_balance(
    client: &crate::Client,
    customer: String,
    params: PostTestHelpersCustomersCustomerFundCashBalanceParams,
) -> crate::Response<crate::generated::CustomerCashBalanceTransaction> {
    client.post_form(
        &format!("/test_helpers/customers/{customer}/fund_cash_balance", customer = customer),
        params,
    )
}

pub fn post_customers_customer_funding_instructions(
    client: &crate::Client,
    customer: String,
    params: PostCustomersCustomerFundingInstructionsParams,
) -> crate::Response<crate::generated::FundingInstructions> {
    client.post_form(
        &format!("/customers/{customer}/funding_instructions", customer = customer),
        params,
    )
}

pub fn delete_customers_customer_discount(
    client: &crate::Client,
    customer: String,
) -> crate::Response<crate::generated::DeletedDiscount> {
    client.delete(&format!("/customers/{customer}/discount", customer = customer))
}
