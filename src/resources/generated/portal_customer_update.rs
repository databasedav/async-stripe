#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    pub allowed_updates: Vec<PortalCustomerUpdateAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl PortalCustomerUpdateAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Address => "address",
            Self::Email => "email",
            Self::Phone => "phone",
            Self::Shipping => "shipping",
            Self::TaxId => "tax_id",
        }
    }
}

impl AsRef<str> for PortalCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalCustomerUpdateAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalCustomerUpdateAllowedUpdates {
    fn default() -> Self {
        Self::Address
    }
}
