/// Issue a credit note to adjust an invoice's amount after the invoice is finalized.
///
/// Related guide: [Credit Notes](https://stripe.com/docs/billing/invoices/credit-notes).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CreditNote {
    /// The integer amount in %s representing the total amount of the credit note, including tax.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the customer.
    pub customer: Vec<crate::generated::Customer>,

    /// Customer balance transaction related to this credit note.
    pub customer_balance_transaction: Option<Vec<crate::generated::CustomerBalanceTransaction>>,

    /// The integer amount in %s representing the total amount of discount that was credited.
    pub discount_amount: i64,

    /// The aggregate amounts calculated per discount for all line items.
    pub discount_amounts: Vec<crate::generated::DiscountsResourceDiscountAmount>,

    /// Unique identifier for the object.
    pub id: String,

    /// ID of the invoice.
    pub invoice: Vec<crate::generated::Invoice>,

    /// Line items that make up the credit note.
    pub lines: crate::params::List<crate::generated::CreditNoteLineItem>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Customer-facing text that appears on the credit note PDF.
    pub memo: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// A unique number that identifies this particular credit note and appears on the PDF of the credit note and its associated invoice.
    pub number: String,

    /// Amount that was credited outside of Stripe.
    pub out_of_band_amount: Option<i64>,

    /// The link to download the PDF of the credit note.
    pub pdf: String,

    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    pub reason: Option<CreditNoteReason>,

    /// Refund related to this credit note.
    pub refund: Option<Vec<crate::generated::Refund>>,

    /// Status of this credit note, one of `issued` or `void`.
    ///
    /// Learn more about [voiding credit notes](https://stripe.com/docs/billing/invoices/credit-notes#voiding).
    pub status: CreditNoteStatus,

    /// The integer amount in %s representing the amount of the credit note, excluding exclusive tax and invoice level discounts.
    pub subtotal: i64,

    /// The integer amount in %s representing the amount of the credit note, excluding all tax and invoice level discounts.
    pub subtotal_excluding_tax: Option<i64>,

    /// The aggregate amounts calculated per tax rate for all line items.
    pub tax_amounts: Vec<crate::generated::CreditNoteTaxAmount>,

    /// The integer amount in %s representing the total amount of the credit note, including tax and all discount.
    pub total: i64,

    /// The integer amount in %s representing the total amount of the credit note, excluding tax, but including discounts.
    pub total_excluding_tax: Option<i64>,

    /// Type of this credit note, one of `pre_payment` or `post_payment`.
    ///
    /// A `pre_payment` credit note means it was issued when the invoice was open.
    /// A `post_payment` credit note means it was issued when the invoice was paid.
    #[serde(rename = "type")]
    pub type_: CreditNoteType,

    /// The time that the credit note was voided.
    pub voided_at: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCreditNotesParams {
    /// The integer amount in cents (or local equivalent) representing the total amount of the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The integer amount in cents (or local equivalent) representing the amount to credit the customer's balance, which will be automatically applied to their next invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// ID of the invoice.
    pub invoice: String,

    /// Line items that make up the credit note.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<PostCreditNotesParamsLines>>,

    /// The credit note's memo appears on the credit note PDF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The integer amount in cents (or local equivalent) representing the amount that is credited outside of Stripe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,

    /// Reason for issuing this credit note, one of `duplicate`, `fraudulent`, `order_change`, or `product_unsatisfactory`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<PostCreditNotesParamsReason>,

    /// ID of an existing refund to link this credit note to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<String>,

    /// The integer amount in cents (or local equivalent) representing the amount to refund.
    ///
    /// If set, a refund will be created for the charge associated with the invoice.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesPreviewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub invoice: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<GetCreditNotesPreviewParamsLines>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<GetCreditNotesPreviewParamsReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesParams {
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
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCreditNotesIdParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Credit note memo.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCreditNotesIdVoidParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesPreviewLinesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub invoice: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub lines: Option<Vec<GetCreditNotesPreviewLinesParamsLines>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub memo: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub out_of_band_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<GetCreditNotesPreviewLinesParamsReason>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl CreditNoteReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for CreditNoteReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CreditNoteReason {
    fn default() -> Self {
        Self::Duplicate
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteStatus {
    Issued,
    Void,
}

impl CreditNoteStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Issued => "issued",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for CreditNoteStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CreditNoteStatus {
    fn default() -> Self {
        Self::Issued
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CreditNoteType {
    PostPayment,
    PrePayment,
}

impl CreditNoteType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::PostPayment => "post_payment",
            Self::PrePayment => "pre_payment",
        }
    }
}

impl AsRef<str> for CreditNoteType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CreditNoteType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CreditNoteType {
    fn default() -> Self {
        Self::PostPayment
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCreditNotesParamsLines {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,

    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: PostCreditNotesParamsLinesType,

    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
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
pub enum PostCreditNotesParamsReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl PostCreditNotesParamsReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for PostCreditNotesParamsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCreditNotesParamsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCreditNotesParamsReason {
    fn default() -> Self {
        Self::Duplicate
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesPreviewParamsLines {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,

    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: GetCreditNotesPreviewParamsLinesType,

    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
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
pub enum GetCreditNotesPreviewParamsReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl GetCreditNotesPreviewParamsReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for GetCreditNotesPreviewParamsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetCreditNotesPreviewParamsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetCreditNotesPreviewParamsReason {
    fn default() -> Self {
        Self::Duplicate
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetCreditNotesPreviewLinesParamsLines {
    /// The line item amount to credit.
    ///
    /// Only valid when `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The description of the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The invoice line item to credit.
    ///
    /// Only valid when the `type` is `invoice_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_line_item: Option<String>,

    /// The line item quantity to credit.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the credit note line item.
    ///
    /// Only valid when the `type` is `custom_line_item`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,

    /// Type of the credit note line item, one of `invoice_line_item` or `custom_line_item`.
    #[serde(rename = "type")]
    pub type_: GetCreditNotesPreviewLinesParamsLinesType,

    /// The integer unit amount in cents (or local equivalent) of the credit note line item.
    ///
    /// This `unit_amount` will be multiplied by the quantity to get the full amount to credit for this line item.
    /// Only valid when `type` is `custom_line_item`.
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
pub enum GetCreditNotesPreviewLinesParamsReason {
    Duplicate,
    Fraudulent,
    OrderChange,
    ProductUnsatisfactory,
}

impl GetCreditNotesPreviewLinesParamsReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Duplicate => "duplicate",
            Self::Fraudulent => "fraudulent",
            Self::OrderChange => "order_change",
            Self::ProductUnsatisfactory => "product_unsatisfactory",
        }
    }
}

impl AsRef<str> for GetCreditNotesPreviewLinesParamsReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetCreditNotesPreviewLinesParamsReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetCreditNotesPreviewLinesParamsReason {
    fn default() -> Self {
        Self::Duplicate
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCreditNotesParamsLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl PostCreditNotesParamsLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for PostCreditNotesParamsLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCreditNotesParamsLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCreditNotesParamsLinesType {
    fn default() -> Self {
        Self::CustomLineItem
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetCreditNotesPreviewParamsLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl GetCreditNotesPreviewParamsLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for GetCreditNotesPreviewParamsLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetCreditNotesPreviewParamsLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetCreditNotesPreviewParamsLinesType {
    fn default() -> Self {
        Self::CustomLineItem
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetCreditNotesPreviewLinesParamsLinesType {
    CustomLineItem,
    InvoiceLineItem,
}

impl GetCreditNotesPreviewLinesParamsLinesType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomLineItem => "custom_line_item",
            Self::InvoiceLineItem => "invoice_line_item",
        }
    }
}

impl AsRef<str> for GetCreditNotesPreviewLinesParamsLinesType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetCreditNotesPreviewLinesParamsLinesType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetCreditNotesPreviewLinesParamsLinesType {
    fn default() -> Self {
        Self::CustomLineItem
    }
}
pub fn post_credit_notes(
    client: &crate::Client,
    params: PostCreditNotesParams,
) -> crate::Response<crate::generated::CreditNote> {
    client.post_form("/credit_notes", params)
}

pub fn get_credit_notes_preview(
    client: &crate::Client,
    params: GetCreditNotesPreviewParams,
) -> crate::Response<crate::generated::CreditNote> {
    client.get_query("/credit_notes/preview", params)
}

pub fn get_credit_notes_id(
    client: &crate::Client,
    id: String,
    params: GetCreditNotesIdParams,
) -> crate::Response<crate::generated::CreditNote> {
    client.get_query(&format!("/credit_notes/{id}", id = id), params)
}

pub fn get_credit_notes(
    client: &crate::Client,
    params: GetCreditNotesParams,
) -> crate::Response<crate::params::List<crate::generated::CreditNote>> {
    client.get_query("/credit_notes", params)
}

pub fn post_credit_notes_id(
    client: &crate::Client,
    id: String,
    params: PostCreditNotesIdParams,
) -> crate::Response<crate::generated::CreditNote> {
    client.post_form(&format!("/credit_notes/{id}", id = id), params)
}

pub fn post_credit_notes_id_void(
    client: &crate::Client,
    id: String,
    params: PostCreditNotesIdVoidParams,
) -> crate::Response<crate::generated::CreditNote> {
    client.post_form(&format!("/credit_notes/{id}/void", id = id), params)
}

pub fn get_credit_notes_preview_lines(
    client: &crate::Client,
    params: GetCreditNotesPreviewLinesParams,
) -> crate::Response<crate::params::List<crate::generated::CreditNoteLineItem>> {
    client.get_query("/credit_notes/preview/lines", params)
}
