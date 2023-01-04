/// Transactions represent changes to a [FinancialAccount's](https://stripe.com/docs/api#financial_accounts) balance.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryTransaction {
    /// Amount (in cents) transferred.
    pub amount: i64,

    pub balance_impact: crate::generated::TreasuryTransactionsResourceBalanceImpact,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: String,

    /// A list of TransactionEntries that are part of this Transaction.
    ///
    /// This cannot be expanded in any list endpoints.
    pub entries: crate::params::List<crate::generated::TreasuryTransactionEntry>,

    /// The FinancialAccount associated with this object.
    pub financial_account: String,

    /// ID of the flow that created the Transaction.
    pub flow: Option<String>,

    /// Details of the flow that created the Transaction.
    pub flow_details: Option<crate::generated::TreasuryTransactionsResourceFlowDetails>,

    /// Type of the flow that created the Transaction.
    pub flow_type: TreasuryTransactionFlowType,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Status of the Transaction.
    pub status: TreasuryTransactionStatus,

    pub status_transitions:
        crate::generated::TreasuryTransactionsResourceAbstractTransactionResourceStatusTransitions,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryTransactionsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<GetTreasuryTransactionsParamsOrderBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTreasuryTransactionsParamsStatus>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_transitions: Option<GetTreasuryTransactionsParamsStatusTransitions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionFlowType {
    CreditReversal,
    DebitReversal,
    InboundTransfer,
    IssuingAuthorization,
    Other,
    OutboundPayment,
    OutboundTransfer,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionFlowType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::IssuingAuthorization => "issuing_authorization",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundTransfer => "outbound_transfer",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryTransactionFlowType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionStatus {
    Open,
    Posted,
    Void,
}

impl TreasuryTransactionStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Posted => "posted",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for TreasuryTransactionStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryTransactionStatus {
    fn default() -> Self {
        Self::Open
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryTransactionsParamsOrderBy {
    Created,
    PostedAt,
}

impl GetTreasuryTransactionsParamsOrderBy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::PostedAt => "posted_at",
        }
    }
}

impl AsRef<str> for GetTreasuryTransactionsParamsOrderBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryTransactionsParamsOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryTransactionsParamsOrderBy {
    fn default() -> Self {
        Self::Created
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryTransactionsParamsStatus {
    Open,
    Posted,
    Void,
}

impl GetTreasuryTransactionsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Open => "open",
            Self::Posted => "posted",
            Self::Void => "void",
        }
    }
}

impl AsRef<str> for GetTreasuryTransactionsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryTransactionsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryTransactionsParamsStatus {
    fn default() -> Self {
        Self::Open
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryTransactionsParamsStatusTransitions {
    /// Returns Transactions with `posted_at` within the specified range.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub posted_at: Option<crate::params::RangeQueryTs>,
}

pub fn get_treasury_transactions_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryTransactionsIdParams,
) -> crate::Response<crate::generated::TreasuryTransaction> {
    client.get_query(&format!("/treasury/transactions/{id}", id = id), params)
}

pub fn get_treasury_transactions(
    client: &crate::Client,
    params: GetTreasuryTransactionsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryTransaction>> {
    client.get_query("/treasury/transactions", params)
}
