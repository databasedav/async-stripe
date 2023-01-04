#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OutboundPaymentsPaymentMethodDetails {
    pub billing_details: crate::generated::TreasurySharedResourceBillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account:
        Option<crate::generated::OutboundPaymentsPaymentMethodDetailsFinancialAccount>,

    /// The type of the payment method used in the OutboundPayment.
    #[serde(rename = "type")]
    pub type_: OutboundPaymentsPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::generated::OutboundPaymentsPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OutboundPaymentsPaymentMethodDetailsType {
    FinancialAccount,
    UsBankAccount,
}

impl OutboundPaymentsPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::FinancialAccount => "financial_account",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str> for OutboundPaymentsPaymentMethodDetailsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OutboundPaymentsPaymentMethodDetailsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OutboundPaymentsPaymentMethodDetailsType {
    fn default() -> Self {
        Self::FinancialAccount
    }
}
