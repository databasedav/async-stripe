#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PortalSubscriptionUpdate {
    /// The types of subscription updates that are supported for items listed in the `products` attribute.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates: Vec<PortalSubscriptionUpdateDefaultAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// The list of products that support subscription updates.
    pub products: Option<Vec<crate::generated::PortalSubscriptionUpdateProduct>>,

    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    pub proration_behavior: PortalSubscriptionUpdateProrationBehavior,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl PortalSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
            Self::PromotionCode => "promotion_code",
            Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalSubscriptionUpdateDefaultAllowedUpdates {
    fn default() -> Self {
        Self::Price
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PortalSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PortalSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PortalSubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PortalSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PortalSubscriptionUpdateProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
