/// TransactionEntries represent individual units of money movements within a single [Transaction](https://stripe.com/docs/api#transactions).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryTransactionEntry {
    pub balance_impact: crate::generated::TreasuryTransactionsResourceBalanceImpact,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// When the TransactionEntry will impact the FinancialAccount's balance.
    pub effective_at: crate::params::Timestamp,

    /// The FinancialAccount associated with this object.
    pub financial_account: String,

    /// Token of the flow associated with the TransactionEntry.
    pub flow: Option<String>,

    /// Details of the flow associated with the TransactionEntry.
    pub flow_details: Option<crate::generated::TreasuryTransactionsResourceFlowDetails>,

    /// Type of the flow associated with the TransactionEntry.
    pub flow_type: TreasuryTransactionEntryFlowType,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The Transaction associated with this object.
    pub transaction: Vec<crate::generated::TreasuryTransaction>,

    /// The specific money movement that generated the TransactionEntry.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionEntryType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryTransactionEntriesIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryTransactionEntriesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub effective_at: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_by: Option<GetTreasuryTransactionEntriesParamsOrderBy>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transaction: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryFlowType {
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

impl TreasuryTransactionEntryFlowType {
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

impl AsRef<str> for TreasuryTransactionEntryFlowType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionEntryFlowType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryTransactionEntryFlowType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionEntryType {
    CreditReversal,
    CreditReversalPosting,
    DebitReversal,
    InboundTransfer,
    InboundTransferReturn,
    IssuingAuthorizationHold,
    IssuingAuthorizationRelease,
    Other,
    OutboundPayment,
    OutboundPaymentCancellation,
    OutboundPaymentFailure,
    OutboundPaymentPosting,
    OutboundPaymentReturn,
    OutboundTransfer,
    OutboundTransferCancellation,
    OutboundTransferFailure,
    OutboundTransferPosting,
    OutboundTransferReturn,
    ReceivedCredit,
    ReceivedDebit,
}

impl TreasuryTransactionEntryType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::CreditReversalPosting => "credit_reversal_posting",
            Self::DebitReversal => "debit_reversal",
            Self::InboundTransfer => "inbound_transfer",
            Self::InboundTransferReturn => "inbound_transfer_return",
            Self::IssuingAuthorizationHold => "issuing_authorization_hold",
            Self::IssuingAuthorizationRelease => "issuing_authorization_release",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::OutboundPaymentCancellation => "outbound_payment_cancellation",
            Self::OutboundPaymentFailure => "outbound_payment_failure",
            Self::OutboundPaymentPosting => "outbound_payment_posting",
            Self::OutboundPaymentReturn => "outbound_payment_return",
            Self::OutboundTransfer => "outbound_transfer",
            Self::OutboundTransferCancellation => "outbound_transfer_cancellation",
            Self::OutboundTransferFailure => "outbound_transfer_failure",
            Self::OutboundTransferPosting => "outbound_transfer_posting",
            Self::OutboundTransferReturn => "outbound_transfer_return",
            Self::ReceivedCredit => "received_credit",
            Self::ReceivedDebit => "received_debit",
        }
    }
}

impl AsRef<str> for TreasuryTransactionEntryType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionEntryType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryTransactionEntryType {
    fn default() -> Self {
        Self::CreditReversal
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryTransactionEntriesParamsOrderBy {
    Created,
    EffectiveAt,
}

impl GetTreasuryTransactionEntriesParamsOrderBy {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Created => "created",
            Self::EffectiveAt => "effective_at",
        }
    }
}

impl AsRef<str> for GetTreasuryTransactionEntriesParamsOrderBy {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryTransactionEntriesParamsOrderBy {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryTransactionEntriesParamsOrderBy {
    fn default() -> Self {
        Self::Created
    }
}
pub fn get_treasury_transaction_entries_id(
    client: &crate::Client,
    id: String,
    params: GetTreasuryTransactionEntriesIdParams,
) -> crate::Response<crate::generated::TreasuryTransactionEntry> {
    client.get_query(&format!("/treasury/transaction_entries/{id}", id = id), params)
}

pub fn get_treasury_transaction_entries(
    client: &crate::Client,
    params: GetTreasuryTransactionEntriesParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryTransactionEntry>> {
    client.get_query("/treasury/transaction_entries", params)
}
