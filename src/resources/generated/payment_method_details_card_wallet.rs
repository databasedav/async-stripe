#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsCardWallet {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amex_express_checkout:
        Option<crate::generated::PaymentMethodDetailsCardWalletAmexExpressCheckout>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub apple_pay: Option<crate::generated::PaymentMethodDetailsCardWalletApplePay>,

    /// (For tokenized numbers only.) The last four digits of the device account number.
    pub dynamic_last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub google_pay: Option<crate::generated::PaymentMethodDetailsCardWalletGooglePay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub masterpass: Option<crate::generated::PaymentMethodDetailsCardWalletMasterpass>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub samsung_pay: Option<crate::generated::PaymentMethodDetailsCardWalletSamsungPay>,

    /// The type of the card wallet, one of `amex_express_checkout`, `apple_pay`, `google_pay`, `masterpass`, `samsung_pay`, or `visa_checkout`.
    ///
    /// An additional hash is included on the Wallet subhash with a name matching this value.
    /// It contains additional information specific to the card wallet type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodDetailsCardWalletType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub visa_checkout: Option<crate::generated::PaymentMethodDetailsCardWalletVisaCheckout>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsCardWalletType {
    AmexExpressCheckout,
    ApplePay,
    GooglePay,
    Masterpass,
    SamsungPay,
    VisaCheckout,
}

impl PaymentMethodDetailsCardWalletType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AmexExpressCheckout => "amex_express_checkout",
            Self::ApplePay => "apple_pay",
            Self::GooglePay => "google_pay",
            Self::Masterpass => "masterpass",
            Self::SamsungPay => "samsung_pay",
            Self::VisaCheckout => "visa_checkout",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsCardWalletType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsCardWalletType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodDetailsCardWalletType {
    fn default() -> Self {
        Self::AmexExpressCheckout
    }
}
