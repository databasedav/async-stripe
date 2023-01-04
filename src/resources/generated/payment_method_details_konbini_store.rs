#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetailsKonbiniStore {
    /// The name of the convenience store chain where the payment was completed.
    pub chain: Option<PaymentMethodDetailsKonbiniStoreChain>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodDetailsKonbiniStoreChain {
    Familymart,
    Lawson,
    Ministop,
    Seicomart,
}

impl PaymentMethodDetailsKonbiniStoreChain {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Familymart => "familymart",
            Self::Lawson => "lawson",
            Self::Ministop => "ministop",
            Self::Seicomart => "seicomart",
        }
    }
}

impl AsRef<str> for PaymentMethodDetailsKonbiniStoreChain {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodDetailsKonbiniStoreChain {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodDetailsKonbiniStoreChain {
    fn default() -> Self {
        Self::Familymart
    }
}
