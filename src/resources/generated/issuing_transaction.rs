/// Any use of an [issued card](https://stripe.com/docs/issuing) that results in funds entering or leaving
/// your Stripe account, such as a completed purchase or refund, is represented by an Issuing
/// `Transaction` object.
///
/// Related guide: [Issued Card Transactions](https://stripe.com/docs/issuing/purchases/transactions).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingTransaction {
    /// The transaction amount, which will be reflected in your balance.
    ///
    /// This amount is in your currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<crate::generated::IssuingTransactionAmountDetails>,

    /// The `Authorization` object that led to this transaction.
    pub authorization: Option<Vec<crate::generated::IssuingAuthorization>>,

    /// ID of the [balance transaction](https://stripe.com/docs/api/balance_transactions) associated with this transaction.
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// The card used to make this transaction.
    pub card: Vec<crate::generated::IssuingCard>,

    /// The cardholder to whom this transaction belongs.
    pub cardholder: Option<Vec<crate::generated::IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// If you've disputed the transaction, the ID of the dispute.
    pub dispute: Option<Vec<crate::generated::IssuingDispute>>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The amount that the merchant will receive, denominated in `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    ///
    /// It will be different from `amount` if the merchant is taking payment in a different currency.
    pub merchant_amount: i64,

    /// The currency with which the merchant is taking payment.
    pub merchant_currency: crate::currency::Currency,

    pub merchant_data: crate::generated::IssuingAuthorizationMerchantData,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// Additional purchase information that is optionally provided by the merchant.
    pub purchase_details: Option<crate::generated::IssuingTransactionPurchaseDetails>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this transaction if it was created on a [FinancialAccount](/docs/api/treasury/financial_accounts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<crate::generated::IssuingTransactionTreasury>,

    /// The nature of the transaction.
    #[serde(rename = "type")]
    pub type_: IssuingTransactionType,

    /// The digital wallet used for this transaction.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<IssuingTransactionWallet>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GetIssuingTransactionsParamsType>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingTransactionsTransactionParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingTransactionsTransactionParams {
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
pub enum IssuingTransactionType {
    Capture,
    Refund,
}

impl IssuingTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Refund => "refund",
        }
    }
}

impl AsRef<str> for IssuingTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingTransactionType {
    fn default() -> Self {
        Self::Capture
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingTransactionWallet {
    ApplePay,
    GooglePay,
    SamsungPay,
}

impl IssuingTransactionWallet {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ApplePay => "apple_pay",
            Self::GooglePay => "google_pay",
            Self::SamsungPay => "samsung_pay",
        }
    }
}

impl AsRef<str> for IssuingTransactionWallet {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingTransactionWallet {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingTransactionWallet {
    fn default() -> Self {
        Self::ApplePay
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIssuingTransactionsParamsType {
    Capture,
    Refund,
}

impl GetIssuingTransactionsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Capture => "capture",
            Self::Refund => "refund",
        }
    }
}

impl AsRef<str> for GetIssuingTransactionsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIssuingTransactionsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIssuingTransactionsParamsType {
    fn default() -> Self {
        Self::Capture
    }
}
pub fn get_issuing_transactions(
    client: &crate::Client,
    params: GetIssuingTransactionsParams,
) -> crate::Response<crate::params::List<crate::generated::IssuingTransaction>> {
    client.get_query("/issuing/transactions", params)
}

pub fn get_issuing_transactions_transaction(
    client: &crate::Client,
    transaction: String,
    params: GetIssuingTransactionsTransactionParams,
) -> crate::Response<crate::generated::IssuingTransaction> {
    client.get_query(
        &format!("/issuing/transactions/{transaction}", transaction = transaction),
        params,
    )
}

pub fn post_issuing_transactions_transaction(
    client: &crate::Client,
    transaction: String,
    params: PostIssuingTransactionsTransactionParams,
) -> crate::Response<crate::generated::IssuingTransaction> {
    client.post_form(
        &format!("/issuing/transactions/{transaction}", transaction = transaction),
        params,
    )
}
