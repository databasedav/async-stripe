#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerTax {
    /// Surfaces if automatic tax computation is possible given the current customer location information.
    pub automatic_tax: CustomerTaxAutomaticTax,

    /// A recent IP address of the customer used for tax reporting and tax location inference.
    pub ip_address: Option<String>,

    /// The customer's location as identified by Stripe Tax.
    pub location: Option<crate::generated::CustomerTaxLocation>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxAutomaticTax {
    Failed,
    NotCollecting,
    Supported,
    UnrecognizedLocation,
}

impl CustomerTaxAutomaticTax {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::NotCollecting => "not_collecting",
            Self::Supported => "supported",
            Self::UnrecognizedLocation => "unrecognized_location",
        }
    }
}

impl AsRef<str> for CustomerTaxAutomaticTax {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxAutomaticTax {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerTaxAutomaticTax {
    fn default() -> Self {
        Self::Failed
    }
}
