#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalSubscriptionCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<PortalSubscriptionCancellationReasonOptions>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl PortalSubscriptionCancellationReasonOptions {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CustomerService => "customer_service",
            Self::LowQuality => "low_quality",
            Self::MissingFeatures => "missing_features",
            Self::Other => "other",
            Self::SwitchedService => "switched_service",
            Self::TooComplex => "too_complex",
            Self::TooExpensive => "too_expensive",
            Self::Unused => "unused",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalSubscriptionCancellationReasonOptions {
    fn default() -> Self {
        Self::CustomerService
    }
}
