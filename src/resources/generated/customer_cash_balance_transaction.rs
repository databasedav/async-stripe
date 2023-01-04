/// Customers with certain payments enabled have a cash balance, representing funds that were paid
/// by the customer to a merchant, but have not yet been allocated to a payment.
///
/// Cash Balance Transactions represent when funds are moved into or out of this balance.
/// This includes funding by the customer, allocation to payments, and refunds to the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerCashBalanceTransaction {
#[serde(skip_serializing_if = "Option::is_none")]
pub applied_to_payment: Option<crate::generated::CustomerBalanceResourceCashBalanceTransactionResourceAppliedToPaymentTransaction>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
pub currency: crate::currency::Currency,

    /// The customer whose available cash balance changed as a result of this transaction.
pub customer: Vec<crate::generated::Customer>,

    /// The total available cash balance for the specified currency after this transaction was applied.
    ///
    /// Represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
pub ending_balance: i64,

#[serde(skip_serializing_if = "Option::is_none")]
pub funded: Option<crate::generated::CustomerBalanceResourceCashBalanceTransactionResourceFundedTransaction>,

    /// Unique identifier for the object.
pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
pub livemode: bool,

    /// The amount by which the cash balance changed, represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// A positive value represents funds being added to the cash balance, a negative value represents funds being removed from the cash balance.
pub net_amount: i64,

#[serde(skip_serializing_if = "Option::is_none")]
pub refunded_from_payment: Option<crate::generated::CustomerBalanceResourceCashBalanceTransactionResourceRefundedFromPaymentTransaction>,

    /// The type of the cash balance transaction.
    ///
    /// One of `applied_to_payment`, `unapplied_from_payment`, `refunded_from_payment`, `funded`, `return_initiated`, or `return_canceled`.
    /// New types may be added in future.
    /// See [Customer Balance](https://stripe.com/docs/payments/customer-balance#types) to learn more about these types.
#[serde(rename = "type")]
pub type_: CustomerCashBalanceTransactionType,

#[serde(skip_serializing_if = "Option::is_none")]
pub unapplied_from_payment: Option<crate::generated::CustomerBalanceResourceCashBalanceTransactionResourceUnappliedFromPaymentTransaction>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerCashBalanceTransactionsTransactionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerCashBalanceTransactionsParams {
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
pub enum CustomerCashBalanceTransactionType {
    AppliedToPayment,
    Funded,
    RefundedFromPayment,
    ReturnCanceled,
    ReturnInitiated,
    UnappliedFromPayment,
}

impl CustomerCashBalanceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AppliedToPayment => "applied_to_payment",
            Self::Funded => "funded",
            Self::RefundedFromPayment => "refunded_from_payment",
            Self::ReturnCanceled => "return_canceled",
            Self::ReturnInitiated => "return_initiated",
            Self::UnappliedFromPayment => "unapplied_from_payment",
        }
    }
}

impl AsRef<str> for CustomerCashBalanceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerCashBalanceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerCashBalanceTransactionType {
    fn default() -> Self {
        Self::AppliedToPayment
    }
}
pub fn get_customers_customer_cash_balance_transactions_transaction(
    client: &crate::Client,
    customer: String,
    transaction: String,
    params: GetCustomersCustomerCashBalanceTransactionsTransactionParams,
) -> crate::Response<crate::generated::CustomerCashBalanceTransaction> {
    client.get_query(
        &format!(
            "/customers/{customer}/cash_balance_transactions/{transaction}",
            customer = customer,
            transaction = transaction
        ),
        params,
    )
}

pub fn get_customers_customer_cash_balance_transactions(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerCashBalanceTransactionsParams,
) -> crate::Response<crate::params::List<crate::generated::CustomerCashBalanceTransaction>> {
    client.get_query(
        &format!("/customers/{customer}/cash_balance_transactions", customer = customer),
        params,
    )
}
