#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OutboundTransfersPaymentMethodDetails {
    pub billing_details: crate::generated::TreasurySharedResourceBillingDetails,

    /// The type of the payment method used in the OutboundTransfer.
    #[serde(rename = "type")]
    pub type_: OutboundTransfersPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::generated::OutboundTransfersPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundTransfersPaymentMethodDetailsType {
    UsBankAccount,
}

impl OutboundTransfersPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for OutboundTransfersPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundTransfersPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OutboundTransfersPaymentMethodDetailsType {
    fn default() -> Self {
        Self::UsBankAccount
    }
}
