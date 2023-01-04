/// A phase describes the plans, coupon, and trialing status of a subscription for a predefined time period.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionSchedulePhaseConfiguration {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    pub add_invoice_items: Vec<crate::generated::SubscriptionScheduleAddInvoiceItem>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account during this phase of the schedule.
    pub application_fee_percent: Option<f64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<crate::generated::SchedulesPhaseAutomaticTax>,

    /// Possible values are `phase_start` or `automatic`.
    ///
    /// If `phase_start` then billing cycle anchor of the subscription is set to the start of the phase when entering the phase.
    /// If `automatic` then the billing cycle anchor is automatically modified as needed when entering the phase.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    pub billing_cycle_anchor: Option<SubscriptionSchedulePhaseConfigurationBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    pub billing_thresholds: Option<crate::generated::SubscriptionBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    pub collection_method: Option<SubscriptionSchedulePhaseConfigurationCollectionMethod>,

    /// ID of the coupon to use during this phase of the subscription schedule.
    pub coupon: Option<Vec<crate::generated::Coupon>>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    pub default_payment_method: Option<Vec<crate::generated::PaymentMethod>>,

    /// The default tax rates to apply to the subscription during this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<crate::generated::TaxRate>>,

    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    pub description: Option<String>,

    /// The end of this phase of the subscription schedule.
    pub end_date: crate::params::Timestamp,

    /// The invoice settings applicable during this phase.
    pub invoice_settings: Option<crate::generated::InvoiceSettingSubscriptionScheduleSetting>,

    /// Subscription items to configure the subscription to during this phase of the subscription schedule.
    pub items: Vec<crate::generated::SubscriptionScheduleConfigurationItem>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    ///
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered.
    /// Updating the underlying subscription's `metadata` directly will not affect the current phase's `metadata`.
    pub metadata: Option<crate::params::Metadata>,

    /// The account (if any) the charge was made on behalf of for charges associated with the schedule's subscription.
    ///
    /// See the Connect documentation for details.
    pub on_behalf_of: Option<Vec<crate::generated::Account>>,

    /// If the subscription schedule will prorate when transitioning to this phase.
    ///
    /// Possible values are `create_prorations` and `none`.
    pub proration_behavior: SubscriptionSchedulePhaseConfigurationProrationBehavior,

    /// The start of this phase of the subscription schedule.
    pub start_date: crate::params::Timestamp,

    /// The account (if any) the associated subscription's payments will be attributed to for tax reporting, and where funds from each payment will be transferred to for each of the subscription's invoices.
    pub transfer_data: Option<crate::generated::SubscriptionTransferData>,

    /// When the trial ends within the phase.
    pub trial_end: Option<crate::params::Timestamp>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionSchedulePhaseConfigurationBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl SubscriptionSchedulePhaseConfigurationCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionSchedulePhaseConfigurationCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionSchedulePhaseConfigurationProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl SubscriptionSchedulePhaseConfigurationProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionSchedulePhaseConfigurationProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
