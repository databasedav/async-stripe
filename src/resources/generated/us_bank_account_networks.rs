#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct UsBankAccountNetworks {
    /// The preferred network.
    pub preferred: Option<String>,

    /// All supported networks.
    pub supported: Vec<UsBankAccountNetworksSupported>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum UsBankAccountNetworksSupported {
    Ach,
    UsDomesticWire,
}

impl UsBankAccountNetworksSupported {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::UsDomesticWire => "us_domestic_wire",
        }
    }
}

impl AsRef<str> for UsBankAccountNetworksSupported {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for UsBankAccountNetworksSupported {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for UsBankAccountNetworksSupported {
    fn default() -> Self {
        Self::Ach
    }
}
