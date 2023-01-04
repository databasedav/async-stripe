/// FinancialAddresses contain identifying information that resolves to a FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccountsResourceFinancialAddress {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<crate::generated::TreasuryFinancialAccountsResourceAbaRecord>,

    /// The list of networks that the address supports.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub supported_networks:
        Option<Vec<TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks>>,

    /// The type of financial address.
    #[serde(rename = "type")]
    pub type_: TreasuryFinancialAccountsResourceFinancialAddressType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    Ach,
    UsDomesticWire,
}

impl TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceFinancialAddressSupportedNetworks {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountsResourceFinancialAddressType {
    Aba,
}

impl TreasuryFinancialAccountsResourceFinancialAddressType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Aba => "aba",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountsResourceFinancialAddressType {
    fn default() -> Self {
        Self::Aba
    }
}
