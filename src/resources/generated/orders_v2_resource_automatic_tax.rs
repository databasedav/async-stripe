#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourceAutomaticTax {
    /// Whether Stripe automatically computes tax on this Order.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this Order.
    pub status: Option<OrdersV2ResourceAutomaticTaxStatus>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl OrdersV2ResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl AsRef<str> for OrdersV2ResourceAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersV2ResourceAutomaticTaxStatus {
    fn default() -> Self {
        Self::Complete
    }
}
