/// A customer's `Cash balance` represents real funds.
///
/// Customers can add funds to their cash balance by sending a bank transfer.
/// These funds can be used for payment and can eventually be paid out to your bank account.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CashBalance {
    /// A hash of all cash balances available to this customer.
    ///
    /// You cannot delete a customer with any cash balances, even if the balance is 0.
    /// Amounts are represented in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub available: Option<i64>,

    /// The ID of the customer whose cash balance this object represents.
    pub customer: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub settings: crate::generated::CustomerBalanceCustomerBalanceSettings,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCustomersCustomerCashBalanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerCashBalanceParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// A hash of settings for this cash balance.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub settings: Option<PostCustomersCustomerCashBalanceParamsSettings>,
}

/// A hash of settings for this cash balance.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCustomersCustomerCashBalanceParamsSettings {
    /// Controls how funds transferred by the customer are applied to payment intents and invoices.
    ///
    /// Valid options are `automatic` or `manual`.
    /// For more information about these reconciliation modes, see [Reconciliation](https://stripe.com/docs/payments/customer-balance/reconciliation).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reconciliation_mode:
        Option<PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode {
    Automatic,
    Manual,
}

impl PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCustomersCustomerCashBalanceParamsSettingsReconciliationMode {
    fn default() -> Self {
        Self::Automatic
    }
}
pub fn get_customers_customer_cash_balance(
    client: &crate::Client,
    customer: String,
    params: GetCustomersCustomerCashBalanceParams,
) -> crate::Response<crate::generated::CashBalance> {
    client.get_query(&format!("/customers/{customer}/cash_balance", customer = customer), params)
}

pub fn post_customers_customer_cash_balance(
    client: &crate::Client,
    customer: String,
    params: PostCustomersCustomerCashBalanceParams,
) -> crate::Response<crate::generated::CashBalance> {
    client.post_form(&format!("/customers/{customer}/cash_balance", customer = customer), params)
}
