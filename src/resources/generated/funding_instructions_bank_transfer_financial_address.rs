/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FundingInstructionsBankTransferFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iban: Option<crate::generated::FundingInstructionsBankTransferIbanRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<crate::generated::FundingInstructionsBankTransferSortCodeRecord>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub spei: Option<crate::generated::FundingInstructionsBankTransferSpeiRecord>,

    /// The payment networks supported by this FinancialAddress.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<FundingInstructionsBankTransferFinancialAddressSupportedNetworks>>,

    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferFinancialAddressType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub zengin: Option<crate::generated::FundingInstructionsBankTransferZenginRecord>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    Bacs,
    Fps,
    Sepa,
    Spei,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bacs => "bacs",
            Self::Fps => "fps",
            Self::Sepa => "sepa",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FundingInstructionsBankTransferFinancialAddressSupportedNetworks {
    fn default() -> Self {
        Self::Bacs
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferFinancialAddressType {
    Iban,
    SortCode,
    Spei,
    Zengin,
}

impl FundingInstructionsBankTransferFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Iban => "iban",
            Self::SortCode => "sort_code",
            Self::Spei => "spei",
            Self::Zengin => "zengin",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FundingInstructionsBankTransferFinancialAddressType {
    fn default() -> Self {
        Self::Iban
    }
}
