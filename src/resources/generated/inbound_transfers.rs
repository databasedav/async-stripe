#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InboundTransfers {
    pub billing_details: crate::generated::TreasurySharedResourceBillingDetails,

    /// The type of the payment method used in the InboundTransfer.
    #[serde(rename = "type")]
    pub type_: InboundTransfersType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::generated::InboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InboundTransfersType {
    UsBankAccount,
}

impl InboundTransfersType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for InboundTransfersType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for InboundTransfersType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InboundTransfersType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}
