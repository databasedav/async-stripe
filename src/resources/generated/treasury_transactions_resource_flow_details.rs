#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryTransactionsResourceFlowDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::generated::TreasuryCreditReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub debit_reversal: Option<crate::generated::TreasuryDebitReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfer: Option<crate::generated::TreasuryInboundTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_authorization: Option<crate::generated::IssuingAuthorization>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::generated::TreasuryOutboundPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfer: Option<crate::generated::TreasuryOutboundTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<crate::generated::TreasuryReceivedCredit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<crate::generated::TreasuryReceivedDebit>,

    /// Type of the flow that created the Transaction.
    ///
    /// Set to the same value as `flow_type`.
    #[serde(rename = "type")]
    pub type_: TreasuryTransactionsResourceFlowDetailsType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryTransactionsResourceFlowDetailsType {
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

impl TreasuryTransactionsResourceFlowDetailsType {
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

impl AsRef<str> for TreasuryTransactionsResourceFlowDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryTransactionsResourceFlowDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryTransactionsResourceFlowDetailsType {
    fn default() -> Self {
        Self::CreditReversal
    }
}
