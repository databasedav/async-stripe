#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetails {
    /// Set when `type` is `balance`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub balance: Option<
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance,
    >,

    pub billing_details: crate::generated::TreasurySharedResourceBillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<crate::generated::ReceivedPaymentMethodDetailsFinancialAccount>,

    /// Set when `type` is `issuing_card`.
    ///
    /// This is an [Issuing Card](https://stripe.com/docs/api#issuing_cards) ID.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing_card: Option<String>,

    /// Polymorphic type matching the originating money movement's source.
    ///
    /// This can be an external account, a Stripe balance, or a FinancialAccount.
    #[serde(rename = "type")]
    pub type_:
        TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account:
        Option<crate::generated::TreasurySharedResourceInitiatingPaymentMethodDetailsUsBankAccount>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    Payments,
}

impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Payments => "payments",
        }
    }
}

impl AsRef<str>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsBalance
{
    fn default() -> Self {
        Self::Payments
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    Balance,
    FinancialAccount,
    IssuingCard,
    Stripe,
    UsBankAccount,
}

impl TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Balance => "balance",
            Self::FinancialAccount => "financial_account",
            Self::IssuingCard => "issuing_card",
            Self::Stripe => "stripe",
            Self::UsBankAccount => "us_bank_account",
        }
    }
}

impl AsRef<str>
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for TreasurySharedResourceInitiatingPaymentMethodDetailsInitiatingPaymentMethodDetailsType
{
    fn default() -> Self {
        Self::Balance
    }
}
