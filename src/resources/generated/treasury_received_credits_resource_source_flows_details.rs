#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryReceivedCreditsResourceSourceFlowsDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit_reversal: Option<crate::generated::TreasuryCreditReversal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payment: Option<crate::generated::TreasuryOutboundPayment>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payout: Option<crate::generated::Payout>,

    /// The type of the source flow that originated the ReceivedCredit.
    #[serde(rename = "type")]
    pub type_: TreasuryReceivedCreditsResourceSourceFlowsDetailsType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    CreditReversal,
    Other,
    OutboundPayment,
    Payout,
}

impl TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CreditReversal => "credit_reversal",
            Self::Other => "other",
            Self::OutboundPayment => "outbound_payment",
            Self::Payout => "payout",
        }
    }
}

impl AsRef<str> for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryReceivedCreditsResourceSourceFlowsDetailsType {
    fn default() -> Self {
        Self::CreditReversal
    }
}
