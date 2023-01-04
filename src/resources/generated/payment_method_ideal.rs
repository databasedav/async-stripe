#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodIdeal {
    /// The customer's bank, if provided.
    ///
    /// Can be one of `abn_amro`, `asn_bank`, `bunq`, `handelsbanken`, `ing`, `knab`, `moneyou`, `rabobank`, `regiobank`, `revolut`, `sns_bank`, `triodos_bank`, or `van_lanschot`.
    pub bank: Option<PaymentMethodIdealBank>,

    /// The Bank Identifier Code of the customer's bank, if the bank was provided.
    pub bic: Option<PaymentMethodIdealBic>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl PaymentMethodIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodIdealBic {
    #[serde(rename = "ABNANL2A")]
    Abnanl2a,
    #[serde(rename = "ASNBNL21")]
    Asnbnl21,
    #[serde(rename = "BUNQNL2A")]
    Bunqnl2a,
    #[serde(rename = "FVLBNL22")]
    Fvlbnl22,
    #[serde(rename = "HANDNL2A")]
    Handnl2a,
    #[serde(rename = "INGBNL2A")]
    Ingbnl2a,
    #[serde(rename = "KNABNL2H")]
    Knabnl2h,
    #[serde(rename = "MOYONL21")]
    Moyonl21,
    #[serde(rename = "RABONL2U")]
    Rabonl2u,
    #[serde(rename = "RBRBNL21")]
    Rbrbnl21,
    #[serde(rename = "REVOLT21")]
    Revolt21,
    #[serde(rename = "SNSBNL2A")]
    Snsbnl2a,
    #[serde(rename = "TRIONL2U")]
    Trionl2u,
}

impl PaymentMethodIdealBic {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Abnanl2a => "ABNANL2A",
            Self::Asnbnl21 => "ASNBNL21",
            Self::Bunqnl2a => "BUNQNL2A",
            Self::Fvlbnl22 => "FVLBNL22",
            Self::Handnl2a => "HANDNL2A",
            Self::Ingbnl2a => "INGBNL2A",
            Self::Knabnl2h => "KNABNL2H",
            Self::Moyonl21 => "MOYONL21",
            Self::Rabonl2u => "RABONL2U",
            Self::Rbrbnl21 => "RBRBNL21",
            Self::Revolt21 => "REVOLT21",
            Self::Snsbnl2a => "SNSBNL2A",
            Self::Trionl2u => "TRIONL2U",
        }
    }
}

impl AsRef<str> for PaymentMethodIdealBic {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodIdealBic {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodIdealBic {
    fn default() -> Self {
        Self::Abnanl2a
    }
}
