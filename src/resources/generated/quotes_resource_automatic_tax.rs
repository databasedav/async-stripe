#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct QuotesResourceAutomaticTax {
    /// Automatically calculate taxes.
    pub enabled: bool,

    /// The status of the most recent automated tax calculation for this quote.
    pub status: Option<QuotesResourceAutomaticTaxStatus>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum QuotesResourceAutomaticTaxStatus {
    Complete,
    Failed,
    RequiresLocationInputs,
}

impl QuotesResourceAutomaticTaxStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Complete => "complete",
            Self::Failed => "failed",
            Self::RequiresLocationInputs => "requires_location_inputs",
        }
    }
}

impl AsRef<str> for QuotesResourceAutomaticTaxStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for QuotesResourceAutomaticTaxStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for QuotesResourceAutomaticTaxStatus {
    fn default() -> Self {
        Self::Complete
    }
}
