/// Each customer has a [`balance`](https://stripe.com/docs/api/customers/object#customer_object-balance) value,
/// which denotes a debit or credit that's automatically applied to their next invoice upon finalization.
/// You may modify the value directly by using the [update customer API](https://stripe.com/docs/api/customers/update),
/// or by creating a Customer Balance Transaction, which increments or decrements the customer's `balance` by the specified `amount`.
///
/// Related guide: [Customer Balance](https://stripe.com/docs/billing/customer/balance) to learn more.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerBalanceTransaction {
    /// The amount of the transaction.
    ///
    /// A negative value is a credit for the customer's balance, and a positive value is a debit to the customer's `balance`.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The ID of the credit note (if any) related to the transaction.
    pub credit_note: Option<Vec<crate::generated::CreditNote>>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the customer the transaction belongs to.
    pub customer: Vec<crate::generated::Customer>,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// The customer's `balance` after the transaction was applied.
    ///
    /// A negative value decreases the amount due on the customer's next invoice.
    /// A positive value increases the amount due on the customer's next invoice.
    pub ending_balance: i64,

    /// Unique identifier for the object.
    pub id: String,

    /// The ID of the invoice (if any) related to the transaction.
    pub invoice: Option<Vec<crate::generated::Invoice>>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// Transaction type: `adjustment`, `applied_to_invoice`, `credit_note`, `initial`, `invoice_too_large`, `invoice_too_small`, `unspent_receiver_credit`, or `unapplied_from_invoice`.
    ///
    /// See the [Customer Balance page](https://stripe.com/docs/billing/customer/balance#types) to learn more about transaction types.
    #[serde(rename = "type")]
    pub type_: CustomerBalanceTransactionType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerBalanceTransactionsTransactionParams {
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
pub struct PostCustomersCustomerBalanceTransactionsParams {
    /// The integer amount in **cents (or local equivalent)** to apply to the customer's credit balance.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    /// Specifies the [`invoice_credit_balance`](https://stripe.com/docs/api/customers/object#customer_object-invoice_credit_balance) that this transaction will apply to.
    /// If the customer's `currency` is not set, it will be updated to this value.
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerBalanceTransactionsTransactionParams {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerBalanceTransactionType {
    Adjustment,
    AppliedToInvoice,
    CreditNote,
    Initial,
    InvoiceTooLarge,
    InvoiceTooSmall,
    Migration,
    UnappliedFromInvoice,
    UnspentReceiverCredit,
}

impl CustomerBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Adjustment => "adjustment",
            Self::AppliedToInvoice => "applied_to_invoice",
            Self::CreditNote => "credit_note",
            Self::Initial => "initial",
            Self::InvoiceTooLarge => "invoice_too_large",
            Self::InvoiceTooSmall => "invoice_too_small",
            Self::Migration => "migration",
            Self::UnappliedFromInvoice => "unapplied_from_invoice",
            Self::UnspentReceiverCredit => "unspent_receiver_credit",
        }
    }
}

impl AsRef<str> for CustomerBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerBalanceTransactionType {
    fn default() -> Self {
        Self::Adjustment
    }
}
pub fn get_customers_customer_balance_transactions_transaction(
    client: &crate::Client,
    customer: String,
    transaction: String,
    params: GetCustomersCustomerBalanceTransactionsTransactionParams,
) -> crate::Response<crate::generated::CustomerBalanceTransaction> {
    client.get_query(
        &format!(
            "/customers/{customer}/balance_transactions/{transaction}",
            customer = customer,
            transaction = transaction
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

pub fn post_customers_customer_balance_transactions(
    client: &crate::Client,
    customer: String,
    params: PostCustomersCustomerBalanceTransactionsParams,
) -> crate::Response<crate::generated::CustomerBalanceTransaction> {
    client.post_form(
        &format!("/customers/{customer}/balance_transactions", customer = customer),
        params,
    )
}

pub fn post_customers_customer_balance_transactions_transaction(
    client: &crate::Client,
    customer: String,
    transaction: String,
    params: PostCustomersCustomerBalanceTransactionsTransactionParams,
) -> crate::Response<crate::generated::CustomerBalanceTransaction> {
    client.post_form(
        &format!(
            "/customers/{customer}/balance_transactions/{transaction}",
            customer = customer,
            transaction = transaction
        ),
        params,
    )
}
