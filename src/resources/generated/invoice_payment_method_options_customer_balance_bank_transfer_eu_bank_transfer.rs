#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransfer {
    /// The desired country code of the bank account information.
    ///
    /// Permitted values include: `DE`, `ES`, `FR`, `IE`, or `NL`.
    pub country: InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "FR")]
    Fr,
    #[serde(rename = "IE")]
    Ie,
    #[serde(rename = "NL")]
    Nl,
}

impl InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::De => "DE",
            Self::Es => "ES",
            Self::Fr => "FR",
            Self::Ie => "IE",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for InvoicePaymentMethodOptionsCustomerBalanceBankTransferEuBankTransferCountry {
    fn default() -> Self {
        Self::De
    }
}
