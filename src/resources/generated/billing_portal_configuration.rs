use std::str::FromStr;
/// A portal configuration describes the functionality and behavior of a portal session.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BillingPortalConfiguration {
    /// Whether the configuration is active and can be used to create portal sessions.
    pub active: bool,

    /// ID of the Connect Application that created the configuration.
    pub application: Option<crate::params::Expandable<crate::generated::Application>>,

    pub business_profile: crate::generated::PortalBusinessProfile,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    pub default_return_url: Option<String>,

    pub features: crate::generated::PortalFeatures,

    /// Unique identifier for the object.
    pub id: String,

    /// Whether the configuration is the default.
    ///
    /// If `true`, this configuration can be managed in the Dashboard and portal sessions will use this configuration unless it is overriden when creating the session.
    pub is_default: bool,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    pub login_page: crate::generated::PortalLoginPage,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// Time at which the object was last updated.
    ///
    /// Measured in seconds since the Unix epoch.
    pub updated: crate::params::Timestamp,
}

impl crate::params::Object for BillingPortalConfiguration {
    type Id = crate::ids::BillingPortalConfigurationId;
    fn id(&self) -> Self::Id {
        crate::ids::BillingPortalConfigurationId::from_str(&self.id).unwrap()
    }
    fn object(&self) -> &'static str {
        "billing_portal_configuration"
    }
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetBillingPortalConfigurationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParams {
    /// The business information shown to customers in the portal.
    pub business_profile: PostBillingPortalConfigurationsParamsBusinessProfile,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Information about the features available in the portal.
    pub features: PostBillingPortalConfigurationsParamsFeatures,

    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<PostBillingPortalConfigurationsParamsLoginPage>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParams {
    /// Whether the configuration is active and can be used to create portal sessions.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The business information shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub business_profile: Option<PostBillingPortalConfigurationsConfigurationParamsBusinessProfile>,

    /// The default URL to redirect customers to when they click on the portal's link to return to your website.
    ///
    /// This can be [overriden](https://stripe.com/docs/api/customer_portal/sessions/create#create_portal_session-return_url) when creating the session.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_return_url: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Information about the features available in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PostBillingPortalConfigurationsConfigurationParamsFeatures>,

    /// The hosted login page for this configuration.
    ///
    /// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub login_page: Option<PostBillingPortalConfigurationsConfigurationParamsLoginPage>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetBillingPortalConfigurationsConfigurationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

/// The business information shown to customers in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}

/// Information about the features available in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeatures {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update: Option<PostBillingPortalConfigurationsParamsFeaturesCustomerUpdate>,

    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history: Option<PostBillingPortalConfigurationsParamsFeaturesInvoiceHistory>,

    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update:
        Option<PostBillingPortalConfigurationsParamsFeaturesPaymentMethodUpdate>,

    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel:
        Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancel>,

    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause: Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionPause>,

    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update:
        Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdate>,
}

/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    pub enabled: bool,
}

/// The business information shown to customers in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsBusinessProfile {
    /// The messaging shown to customers in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headline: Option<String>,

    /// A link to the business’s publicly available privacy policy.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub privacy_policy_url: Option<String>,

    /// A link to the business’s publicly available terms of service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub terms_of_service_url: Option<String>,
}

/// Information about the features available in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeatures {
    /// Information about updating the customer details in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_update:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdate>,

    /// Information about showing the billing history in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_history:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesInvoiceHistory>,

    /// Information about updating payment methods in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method_update:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesPaymentMethodUpdate>,

    /// Information about canceling subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_cancel:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancel>,

    /// Information about pausing subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_pause:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionPause>,

    /// Information about updating subscriptions in the portal.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscription_update:
        Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdate>,
}

/// The hosted login page for this configuration.
///
/// Learn more about the portal login page in our [integration docs](https://stripe.com/docs/billing/subscriptions/integrating-customer-portal#share).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsLoginPage {
    /// Set to `true` to generate a shareable URL [`login_page.url`](https://stripe.com/docs/api/customer_portal/configuration#portal_configuration_object-login_page-url) that will take your customers to a hosted login page for the customer portal.
    ///
    /// Set to `false` to deactivate the `login_page.url`.
    pub enabled: bool,
}

/// Information about updating the customer details in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates:
        Option<Vec<PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates>>,

    /// Whether the feature is enabled.
    pub enabled: bool,
}

/// Information about showing the billing history in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

/// Information about updating payment methods in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

/// Information about canceling subscriptions in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancel {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason:
        Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReason>,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Whether to cancel subscriptions immediately or at the end of the billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mode: Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode>,

    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior>,
}

/// Information about pausing subscriptions in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesSubscriptionPause {
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Information about updating subscriptions in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdate {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
    pub default_allowed_updates:
        Vec<PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates>,

    /// Whether the feature is enabled.
    pub enabled: bool,

    /// The list of products that support subscription updates.
    pub products: Vec<PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProducts>,

    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior:
        Option<PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior>,
}

/// Information about updating the customer details in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdate {
    /// The types of customer updates that are supported.
    ///
    /// When empty, customers are not updateable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_updates: Option<
        Vec<PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates>,
    >,

    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Information about showing the billing history in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesInvoiceHistory {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

/// Information about updating payment methods in the portal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesPaymentMethodUpdate {
    /// Whether the feature is enabled.
    pub enabled: bool,
}

/// Information about canceling subscriptions in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancel {
    /// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[serde(skip_serializing_if = "Option::is_none")]
pub cancellation_reason: Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReason>,

    /// Whether the feature is enabled.
#[serde(skip_serializing_if = "Option::is_none")]
pub enabled: Option<bool>,

    /// Whether to cancel subscriptions immediately or at the end of the billing period.
#[serde(skip_serializing_if = "Option::is_none")]
pub mode: Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode>,

    /// Whether to create prorations when canceling subscriptions.
    ///
    /// Possible values are `none` and `create_prorations`, which is only compatible with `mode=immediately`.
    /// No prorations are generated when canceling a subscription at the end of its natural billing period.
#[serde(skip_serializing_if = "Option::is_none")]
pub proration_behavior: Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior>,
}

/// Information about pausing subscriptions in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionPause {
    /// Whether the feature is enabled.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<bool>,
}

/// Information about updating subscriptions in the portal.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdate {
    /// The types of subscription updates that are supported.
    ///
    /// When empty, subscriptions are not updateable.
#[serde(skip_serializing_if = "Option::is_none")]
pub default_allowed_updates: Option<Vec<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates>>,

    /// Whether the feature is enabled.
#[serde(skip_serializing_if = "Option::is_none")]
pub enabled: Option<bool>,

    /// The list of products that support subscription updates.
#[serde(skip_serializing_if = "Option::is_none")]
pub products: Option<Vec<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProducts>>,

    /// Determines how to handle prorations resulting from subscription updates.
    ///
    /// Valid values are `none`, `create_prorations`, and `always_invoice`.
#[serde(skip_serializing_if = "Option::is_none")]
pub proration_behavior: Option<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates {
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

impl AsRef<str> for PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsParamsFeaturesCustomerUpdateAllowedUpdates {
    fn default() -> Self {
        Self::Address
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
    pub enabled: bool,

    /// Which cancellation reasons will be given as options to the customer.
    pub options: Vec<
        PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl AsRef<str> for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelMode {
    fn default() -> Self {
        Self::AtPeriodEnd
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    Price,
    PromotionCode,
    Quantity,
}

impl PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Price => "price",
            Self::PromotionCode => "promotion_code",
            Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str>
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    fn default() -> Self {
        Self::Price
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProducts {
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: Vec<String>,

    /// The product id.
    pub product: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str>
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsParamsFeaturesSubscriptionUpdateProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates {
    Address,
    Email,
    Phone,
    Shipping,
    TaxId,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates {
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

impl AsRef<str>
    for PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostBillingPortalConfigurationsConfigurationParamsFeaturesCustomerUpdateAllowedUpdates
{
    fn default() -> Self {
        Self::Address
    }
}
/// Whether the cancellation reasons will be collected in the portal and which options are exposed to the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReason {
    /// Whether the feature is enabled.
pub enabled: bool,

    /// Which cancellation reasons will be given as options to the customer.
#[serde(skip_serializing_if = "Option::is_none")]
pub options: Option<Vec<PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode {
    AtPeriodEnd,
    Immediately,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AtPeriodEnd => "at_period_end",
            Self::Immediately => "immediately",
        }
    }
}

impl AsRef<str>
    for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelMode {
    fn default() -> Self {
        Self::AtPeriodEnd
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior
{
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates
{
    Price,
    PromotionCode,
    Quantity,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    pub fn as_str(self) -> &'static str {
        match self {
Self::Price => "price",
Self::PromotionCode => "promotion_code",
Self::Quantity => "quantity",
        }
    }
}

impl AsRef<str> for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateDefaultAllowedUpdates {
    fn default() -> Self {
        Self::Price
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProducts {
    /// The list of price IDs for the product that a subscription can be updated to.
    pub prices: Vec<String>,

    /// The product id.
    pub product: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior
{
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionUpdateProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions {
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions {
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

impl AsRef<str>
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostBillingPortalConfigurationsParamsFeaturesSubscriptionCancelCancellationReasonOptions
{
    fn default() -> Self {
        Self::CustomerService
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions
{
    CustomerService,
    LowQuality,
    MissingFeatures,
    Other,
    SwitchedService,
    TooComplex,
    TooExpensive,
    Unused,
}

impl PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions {
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

impl AsRef<str> for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostBillingPortalConfigurationsConfigurationParamsFeaturesSubscriptionCancelCancellationReasonOptions {
    fn default() -> Self {
        Self::CustomerService
    }
}
pub fn get_billing_portal_configurations(
    client: &crate::Client,
    params: GetBillingPortalConfigurationsParams,
) -> crate::Response<crate::params::List<crate::generated::BillingPortalConfiguration>> {
    client.get_query("/billing_portal/configurations", params)
}

pub fn post_billing_portal_configurations(
    client: &crate::Client,
    params: PostBillingPortalConfigurationsParams,
) -> crate::Response<crate::generated::BillingPortalConfiguration> {
    client.post_form("/billing_portal/configurations", params)
}

pub fn post_billing_portal_configurations_configuration(
    client: &crate::Client,
    configuration: String,
    params: PostBillingPortalConfigurationsConfigurationParams,
) -> crate::Response<crate::generated::BillingPortalConfiguration> {
    client.post_form(
        &format!("/billing_portal/configurations/{configuration}", configuration = configuration),
        params,
    )
}

pub fn get_billing_portal_configurations_configuration(
    client: &crate::Client,
    configuration: String,
    params: GetBillingPortalConfigurationsConfigurationParams,
) -> crate::Response<crate::generated::BillingPortalConfiguration> {
    client.get_query(
        &format!("/billing_portal/configurations/{configuration}", configuration = configuration),
        params,
    )
}
