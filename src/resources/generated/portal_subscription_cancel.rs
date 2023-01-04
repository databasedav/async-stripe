#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalSubscriptionCancel {
    pub cancellation_reason: crate::generated::PortalSubscriptionCancellationReason,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    pub mode: PortalSubscriptionCancelMode,

    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`.
    pub proration_behavior: PortalSubscriptionCancelProrationBehavior,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl PortalSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalSubscriptionCancelMode {
    fn default() -> Self {
        Self::AtPeriodEnd
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PortalSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PortalSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalSubscriptionCancelProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
