#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CheckoutCustomerBalanceBankTransferPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eu_bank_transfer:
        Option<crate::generated::PaymentMethodOptionsCustomerBalanceEuBankAccount>,

    /// List of address types that should be returned in the financial_addresses response.
    ///
    /// If not specified, all valid types will be returned.  Permitted values include: `sort_code`, `zengin`, `iban`, or `spei`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub requested_address_types:
        Option<Vec<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes>>,

    /// The bank transfer type that this PaymentIntent is allowed to use for funding Permitted values include: `eu_bank_transfer`, `gb_bank_transfer`, `jp_bank_transfer`, or `mx_bank_transfer`.
    #[serde(rename = "type")]
    pub type_: Option<CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    Iban,
    Sepa,
    SortCode,
    Spei,
    Zengin,
}

impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::Sepa => "sepa",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsRequestedAddressTypes {
    fn default() -> Self {
        Self::Iban
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    EuBankTransfer,
    GbBankTransfer,
    JpBankTransfer,
    MxBankTransfer,
}

impl CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::GbBankTransfer => "gb_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
            Self::MxBankTransfer => "mx_bank_transfer",
        }
    }
}

impl AsRef<str> for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CheckoutCustomerBalanceBankTransferPaymentMethodOptionsType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}
