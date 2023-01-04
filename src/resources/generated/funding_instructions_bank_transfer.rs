#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FundingInstructionsBankTransfer {
    /// The country of the bank account to fund.
    pub country: String,

    /// A list of financial addresses that can be used to fund a particular balance.
    pub financial_addresses: Vec<crate::generated::FundingInstructionsBankTransferFinancialAddress>,

    /// The bank_transfer type.
    #[serde(rename = "type")]
    pub type_: FundingInstructionsBankTransferType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum FundingInstructionsBankTransferType {
    EuBankTransfer,
    JpBankTransfer,
}

impl FundingInstructionsBankTransferType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::EuBankTransfer => "eu_bank_transfer",
            Self::JpBankTransfer => "jp_bank_transfer",
        }
    }
}

impl AsRef<str> for FundingInstructionsBankTransferType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for FundingInstructionsBankTransferType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for FundingInstructionsBankTransferType {
    fn default() -> Self {
        Self::EuBankTransfer
    }
}
