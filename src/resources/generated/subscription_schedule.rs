/// A subscription schedule allows you to create and manage the lifecycle of a subscription by predefining expected changes.
///
/// Related guide: [Subscription Schedules](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SubscriptionSchedule {
    /// ID of the Connect Application that created the schedule.
    pub application: Option<Vec<crate::generated::Application>>,

    /// Time at which the subscription schedule was canceled.
    ///
    /// Measured in seconds since the Unix epoch.
    pub canceled_at: Option<crate::params::Timestamp>,

    /// Time at which the subscription schedule was completed.
    ///
    /// Measured in seconds since the Unix epoch.
    pub completed_at: Option<crate::params::Timestamp>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Object representing the start and end dates for the current phase of the subscription schedule, if it is `active`.
    pub current_phase: Option<crate::generated::SubscriptionScheduleCurrentPhase>,

    /// ID of the customer who owns the subscription schedule.
    pub customer: Vec<crate::generated::Customer>,

    pub default_settings: crate::generated::SubscriptionSchedulesResourceDefaultSettings,

    /// Behavior of the subscription schedule and underlying subscription when it ends.
    ///
    /// Possible values are `release` and `cancel`.
    pub end_behavior: SubscriptionScheduleEndBehavior,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// Configuration for the subscription schedule's phases.
    pub phases: Vec<crate::generated::SubscriptionSchedulePhaseConfiguration>,

    /// Time at which the subscription schedule was released.
    ///
    /// Measured in seconds since the Unix epoch.
    pub released_at: Option<crate::params::Timestamp>,

    /// ID of the subscription once managed by the subscription schedule (if it is released).
    pub released_subscription: Option<String>,

    /// The present status of the subscription schedule.
    ///
    /// Possible values are `not_started`, `active`, `completed`, `released`, and `canceled`.
    /// You can read more about the different states in our [behavior guide](https://stripe.com/docs/billing/subscriptions/subscription-schedules).
    pub status: SubscriptionScheduleStatus,

    /// ID of the subscription managed by the subscription schedule.
    pub subscription: Option<Vec<crate::generated::Subscription>>,

    /// ID of the test clock this subscription schedule belongs to.
    pub test_clock: Option<Vec<crate::generated::TestHelpersTestClock>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSubscriptionSchedulesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub canceled_at: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub completed_at: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub released_at: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub scheduled: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParams {
    /// The identifier of the customer to create the subscription schedule for.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<PostSubscriptionSchedulesParamsDefaultSettings>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<PostSubscriptionSchedulesParamsEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Migrate an existing subscription to be managed by a subscription schedule.
    ///
    /// If this parameter is set, a subscription schedule will be created using the subscription's item(s), set to auto-renew using the subscription's interval.
    /// When using this parameter, other parameters (such as phase values) cannot be set.
    /// To create a subscription schedule with other modifications, we recommend making two separate API calls.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_subscription: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<PostSubscriptionSchedulesParamsPhases>>,

    /// When the subscription schedule starts.
    ///
    /// We recommend using `now` so that it starts the subscription immediately.
    /// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<PostSubscriptionSchedulesParamsStartDate>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSubscriptionSchedulesScheduleParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParams {
    /// Object representing the subscription schedule's default settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_settings: Option<PostSubscriptionSchedulesScheduleParamsDefaultSettings>,

    /// Configures how the subscription schedule behaves when it ends.
    ///
    /// Possible values are `release` or `cancel` with the default being `release`.
    /// `release` will end the subscription schedule and keep the underlying subscription running.`cancel` will end the subscription schedule and cancel the underlying subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_behavior: Option<PostSubscriptionSchedulesScheduleParamsEndBehavior>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// List representing phases of the subscription schedule.
    ///
    /// Each phase can be customized to have different durations, plans, and coupons.
    /// If there are multiple phases, the `end_date` of one phase will always equal the `start_date` of the next phase.
    /// Note that past phases can be omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phases: Option<Vec<PostSubscriptionSchedulesScheduleParamsPhases>>,

    /// If the update changes the current phase, indicates whether the changes should be prorated.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<PostSubscriptionSchedulesScheduleParamsProrationBehavior>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleCancelParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// If the subscription schedule is `active`, indicates if a final invoice will be generated that contains any un-invoiced metered usage and new/pending proration invoice items.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_now: Option<bool>,

    /// If the subscription schedule is `active`, indicates if the cancellation should be prorated.
    ///
    /// Defaults to `true`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prorate: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleReleaseParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Keep any cancellation on the subscription that the schedule has set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preserve_cancel_date: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl SubscriptionScheduleEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::None => "none",
            Self::Release => "release",
            Self::Renew => "renew",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionScheduleEndBehavior {
    fn default() -> Self {
        Self::Cancel
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SubscriptionScheduleStatus {
    Active,
    Canceled,
    Completed,
    NotStarted,
    Released,
}

impl SubscriptionScheduleStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Completed => "completed",
            Self::NotStarted => "not_started",
            Self::Released => "released",
        }
    }
}

impl AsRef<str> for SubscriptionScheduleStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SubscriptionScheduleStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SubscriptionScheduleStatus {
    fn default() -> Self {
        Self::Active
    }
}
/// Object representing the subscription schedule's default settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsDefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostSubscriptionSchedulesParamsDefaultSettingsAutomaticTax>,

    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    ///
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<PostSubscriptionSchedulesParamsDefaultSettingsBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostSubscriptionSchedulesParamsDefaultSettingsInvoiceSettings>,

    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostSubscriptionSchedulesParamsDefaultSettingsTransferData>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl PostSubscriptionSchedulesParamsEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::None => "none",
            Self::Release => "release",
            Self::Renew => "renew",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsEndBehavior {
    fn default() -> Self {
        Self::Cancel
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items: Option<Vec<PostSubscriptionSchedulesParamsPhasesAddInvoiceItems>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostSubscriptionSchedulesParamsPhasesAutomaticTax>,

    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    ///
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor: Option<PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<PostSubscriptionSchedulesParamsPhasesBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostSubscriptionSchedulesParamsPhasesCollectionMethod>,

    /// The identifier of the coupon to apply to this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The date at which this phase of the subscription schedule ends.
    ///
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<crate::params::Timestamp>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostSubscriptionSchedulesParamsPhasesInvoiceSettings>,

    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<PostSubscriptionSchedulesParamsPhasesItems>,

    /// Integer representing the multiplier applied to the price interval.
    ///
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    ///
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<PostSubscriptionSchedulesParamsPhasesProrationBehavior>,

    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostSubscriptionSchedulesParamsPhasesTransferData>,

    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    /// Sets the phase to trialing from the start date to this date.
    ///
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<crate::params::Timestamp>,
}

/// When the subscription schedule starts.
///
/// We recommend using `now` so that it starts the subscription immediately.
/// You can also use a Unix timestamp to backdate the subscription so that it starts on a past date, or set a future date for the subscription to start on.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsStartDate {
    Timestamp(crate::params::Timestamp),
    Now(PostSubscriptionSchedulesParamsStartDateNow),
}

/// Object representing the subscription schedule's default settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsDefaultSettings {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Default settings for automatic tax computation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsAutomaticTax>,

    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    ///
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds:
        Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method:
        Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings:
        Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsInvoiceSettings>,

    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostSubscriptionSchedulesScheduleParamsDefaultSettingsTransferData>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsEndBehavior {
    Cancel,
    None,
    Release,
    Renew,
}

impl PostSubscriptionSchedulesScheduleParamsEndBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cancel => "cancel",
            Self::None => "none",
            Self::Release => "release",
            Self::Renew => "renew",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsEndBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsEndBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsEndBehavior {
    fn default() -> Self {
        Self::Cancel
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhases {
    /// A list of prices and quantities that will generate invoice items appended to the next invoice for this phase.
    ///
    /// You may pass up to 20 items.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub add_invoice_items:
        Option<Vec<PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItems>>,

    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the application owner's Stripe account.
    /// The request must be made by a platform account on a connected account in order to set an application fee percentage.
    /// For more information, see the application fees [documentation](https://stripe.com/docs/connect/subscriptions#collecting-fees-on-subscriptions).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub application_fee_percent: Option<f64>,

    /// Automatic tax settings for this phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub automatic_tax: Option<PostSubscriptionSchedulesScheduleParamsPhasesAutomaticTax>,

    /// Can be set to `phase_start` to set the anchor to the start of the phase or `automatic` to automatically change it if needed.
    ///
    /// Cannot be set to `phase_start` if this phase specifies a trial.
    /// For more information, see the billing cycle [documentation](https://stripe.com/docs/billing/subscriptions/billing-cycle).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_cycle_anchor:
        Option<PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor>,

    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// Pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<PostSubscriptionSchedulesScheduleParamsPhasesBillingThresholds>,

    /// Either `charge_automatically`, or `send_invoice`.
    ///
    /// When charging automatically, Stripe will attempt to pay the underlying subscription at the end of each billing cycle using the default source attached to the customer.
    /// When sending an invoice, Stripe will email your customer an invoice with payment instructions and mark the subscription as `active`.
    /// Defaults to `charge_automatically` on creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub collection_method: Option<PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod>,

    /// The identifier of the coupon to apply to this phase of the subscription schedule.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// ID of the default payment method for the subscription schedule.
    ///
    /// It must belong to the customer associated with the subscription schedule.
    /// If not set, invoices will use the default payment method in the customer's invoice settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_payment_method: Option<String>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will set the Subscription's [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates), which means they will be the Invoice's [`default_tax_rates`](https://stripe.com/docs/api/invoices/create#create_invoice-default_tax_rates) for any Invoices issued by the Subscription during this Phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_tax_rates: Option<Vec<String>>,

    /// Subscription description, meant to be displayable to the customer.
    ///
    /// Use this field to optionally store an explanation of the subscription.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The date at which this phase of the subscription schedule ends.
    ///
    /// If set, `iterations` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub end_date: Option<PostSubscriptionSchedulesScheduleParamsPhasesEndDate>,

    /// All invoices will be billed using the specified settings.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub invoice_settings: Option<PostSubscriptionSchedulesScheduleParamsPhasesInvoiceSettings>,

    /// List of configuration items, each with an attached price, to apply during this phase of the subscription schedule.
    pub items: Vec<PostSubscriptionSchedulesScheduleParamsPhasesItems>,

    /// Integer representing the multiplier applied to the price interval.
    ///
    /// For example, `iterations=2` applied to a price with `interval=month` and `interval_count=3` results in a phase of duration `2 * 3 months = 6 months`.
    /// If set, `end_date` must not be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub iterations: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to a phase.
    ///
    /// Metadata on a schedule's phase will update the underlying subscription's `metadata` when the phase is entered, adding new keys and replacing existing keys in the subscription's `metadata`.
    /// Individual keys in the subscription's `metadata` can be unset by posting an empty value to them in the phase's `metadata`.
    /// To unset all keys in the subscription's `metadata`, update the subscription directly or unset every key individually from the phase's `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The account on behalf of which to charge, for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub on_behalf_of: Option<String>,

    /// Whether the subscription schedule will create [prorations](https://stripe.com/docs/billing/subscriptions/prorations) when transitioning to this phase.
    ///
    /// The default value is `create_prorations`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub proration_behavior: Option<PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior>,

    /// The date at which this phase of the subscription schedule starts or `now`.
    ///
    /// Must be set on the first phase.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub start_date: Option<PostSubscriptionSchedulesScheduleParamsPhasesStartDate>,

    /// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_data: Option<PostSubscriptionSchedulesScheduleParamsPhasesTransferData>,

    /// If set to true the entire phase is counted as a trial and the customer will not be charged for any fees.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial: Option<bool>,

    /// Sets the phase to trialing from the start date to this date.
    ///
    /// Must be before the phase end date, can not be combined with `trial`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trial_end: Option<PostSubscriptionSchedulesScheduleParamsPhasesTrialEnd>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostSubscriptionSchedulesScheduleParamsProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
/// Default settings for automatic tax computation.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsDefaultSettingsBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsDefaultSettingsBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsDefaultSettingsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsDefaultSettingsInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    ///
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsDefaultSettingsTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesAddInvoiceItems {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceData>,

    /// Quantity for this item.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostSubscriptionSchedulesParamsPhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsPhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    ///
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds: Option<PostSubscriptionSchedulesParamsPhasesItemsBillingThresholds>,

    /// The plan ID to subscribe to.
    ///
    /// You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostSubscriptionSchedulesParamsPhasesItemsPriceData>,

    /// Quantity for the given price.
    ///
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostSubscriptionSchedulesParamsPhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}
/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsStartDateNow {
    Now,
}

impl PostSubscriptionSchedulesParamsStartDateNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsStartDateNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsStartDateNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsStartDateNow {
    fn default() -> Self {
        Self::Now
    }
}
/// Default settings for automatic tax computation.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsDefaultSettingsAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsDefaultSettingsBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsDefaultSettingsCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}
/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsDefaultSettingsInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    ///
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsDefaultSettingsTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItems {
    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceData>,

    /// Quantity for this item.
    ///
    /// Defaults to 1.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// The tax rates which apply to the item.
    ///
    /// When set, the `default_tax_rates` do not apply to this item.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

/// Automatic tax settings for this phase.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesAutomaticTax {
    /// Enabled automatic tax calculation which will automatically compute tax rates on all invoices generated by the subscription.
    pub enabled: bool,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor {
    Automatic,
    PhaseStart,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::PhaseStart => "phase_start",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesBillingCycleAnchor {
    fn default() -> Self {
        Self::Automatic
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesBillingThresholds {
    /// Monetary threshold that triggers the subscription to advance to a new billing period.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_gte: Option<i64>,

    /// Indicates if the `billing_cycle_anchor` should be reset when a threshold is reached.
    ///
    /// If true, `billing_cycle_anchor` will be updated to the date/time the threshold was last reached; otherwise, the value will remain unchanged.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reset_billing_cycle_anchor: Option<bool>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod {
    ChargeAutomatically,
    SendInvoice,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ChargeAutomatically => "charge_automatically",
            Self::SendInvoice => "send_invoice",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesCollectionMethod {
    fn default() -> Self {
        Self::ChargeAutomatically
    }
}

/// The date at which this phase of the subscription schedule ends.
///
/// If set, `iterations` must not be set.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesEndDate {
    Timestamp(crate::params::Timestamp),
    Now(PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow),
}

/// All invoices will be billed using the specified settings.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesInvoiceSettings {
    /// Number of days within which a customer must pay invoices generated by this subscription schedule.
    ///
    /// This value will be `null` for subscription schedules where `billing=charge_automatically`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_until_due: Option<u32>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesItems {
    /// Define thresholds at which an invoice will be sent, and the subscription advanced to a new billing period.
    ///
    /// When updating, pass an empty string to remove previously-defined thresholds.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_thresholds:
        Option<PostSubscriptionSchedulesScheduleParamsPhasesItemsBillingThresholds>,

    /// The plan ID to subscribe to.
    ///
    /// You may specify the same ID in `plan` and `price`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub plan: Option<String>,

    /// The ID of the price object.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price: Option<String>,

    /// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub price_data: Option<PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceData>,

    /// Quantity for the given price.
    ///
    /// Can be set only if the price's `usage_type` is `licensed` and not `metered`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    /// A list of [Tax Rate](https://stripe.com/docs/api/tax_rates) ids.
    ///
    /// These Tax Rates will override the [`default_tax_rates`](https://stripe.com/docs/api/subscriptions/create#create_subscription-default_tax_rates) on the Subscription.
    /// When updating, pass an empty string to remove previously-defined tax rates.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_rates: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior {
    AlwaysInvoice,
    CreateProrations,
    None,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AlwaysInvoice => "always_invoice",
            Self::CreateProrations => "create_prorations",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesProrationBehavior {
    fn default() -> Self {
        Self::AlwaysInvoice
    }
}

/// The date at which this phase of the subscription schedule starts or `now`.
///
/// Must be set on the first phase.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesStartDate {
    Timestamp(crate::params::Timestamp),
    Now(PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow),
}

/// The data with which to automatically create a Transfer for each of the associated subscription's invoices.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesTransferData {
    /// A non-negative decimal between 0 and 100, with at most two decimal places.
    ///
    /// This represents the percentage of the subscription invoice subtotal that will be transferred to the destination account.
    /// By default, the entire amount is transferred to the destination.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_percent: Option<f64>,

    /// ID of an existing, connected Stripe account.
    pub destination: String,
}

/// Sets the phase to trialing from the start date to this date.
///
/// Must be before the phase end date, can not be combined with `trial`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesTrialEnd {
    I64(i64),
    Now(PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow),
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurring,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior: Option<PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow {
    Now,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesEndDateNow {
    fn default() -> Self {
        Self::Now
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesItemsBillingThresholds {
    /// Usage threshold that triggers the subscription to advance to a new billing period.
    pub usage_gte: i64,
}

/// Data used to generate a new [Price](https://stripe.com/docs/api/prices) object inline.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceData {
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The ID of the product that this price will belong to.
    pub product: String,

    /// The recurring components of a price such as `interval` and `interval_count`.
    pub recurring: PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurring,

    /// Specifies whether the price is considered inclusive of taxes or exclusive of taxes.
    ///
    /// One of `inclusive`, `exclusive`, or `unspecified`.
    /// Once specified as either `inclusive` or `exclusive`, it cannot be changed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tax_behavior:
        Option<PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior>,

    /// A positive integer in cents (or local equivalent) (or 0 for a free price) representing how much to charge.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount: Option<i64>,

    /// Same as `unit_amount`, but accepts a decimal value in cents (or local equivalent) with at most 12 decimal places.
    ///
    /// Only one of `unit_amount` and `unit_amount_decimal` can be set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unit_amount_decimal: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow {
    Now,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesStartDateNow {
    fn default() -> Self {
        Self::Now
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow {
    Now,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Now => "now",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesTrialEndNow {
    fn default() -> Self {
        Self::Now
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str>
    for PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesAddInvoiceItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}
/// The recurring components of a price such as `interval` and `interval_count`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurring {
    /// Specifies billing frequency.
    ///
    /// Either `day`, `week`, `month` or `year`.
    pub interval: PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval,

    /// The number of intervals between subscription billings.
    ///
    /// For example, `interval=month` and `interval_count=3` bills every 3 months.
    /// Maximum of one year interval allowed (1 year, 12 months, or 52 weeks).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval_count: Option<u64>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior {
    Exclusive,
    Inclusive,
    Unspecified,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Exclusive => "exclusive",
            Self::Inclusive => "inclusive",
            Self::Unspecified => "unspecified",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataTaxBehavior {
    fn default() -> Self {
        Self::Exclusive
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesParamsPhasesItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval {
    Day,
    Month,
    Week,
    Year,
}

impl PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Day => "day",
            Self::Month => "month",
            Self::Week => "week",
            Self::Year => "year",
        }
    }
}

impl AsRef<str> for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSubscriptionSchedulesScheduleParamsPhasesItemsPriceDataRecurringInterval {
    fn default() -> Self {
        Self::Day
    }
}
pub fn get_subscription_schedules(
    client: &crate::Client,
    params: GetSubscriptionSchedulesParams,
) -> crate::Response<crate::params::List<crate::generated::SubscriptionSchedule>> {
    client.get_query("/subscription_schedules", params)
}

pub fn post_subscription_schedules(
    client: &crate::Client,
    params: PostSubscriptionSchedulesParams,
) -> crate::Response<crate::generated::SubscriptionSchedule> {
    client.post_form("/subscription_schedules", params)
}

pub fn get_subscription_schedules_schedule(
    client: &crate::Client,
    schedule: String,
    params: GetSubscriptionSchedulesScheduleParams,
) -> crate::Response<crate::generated::SubscriptionSchedule> {
    client.get_query(&format!("/subscription_schedules/{schedule}", schedule = schedule), params)
}

pub fn post_subscription_schedules_schedule(
    client: &crate::Client,
    schedule: String,
    params: PostSubscriptionSchedulesScheduleParams,
) -> crate::Response<crate::generated::SubscriptionSchedule> {
    client.post_form(&format!("/subscription_schedules/{schedule}", schedule = schedule), params)
}

pub fn post_subscription_schedules_schedule_cancel(
    client: &crate::Client,
    schedule: String,
    params: PostSubscriptionSchedulesScheduleCancelParams,
) -> crate::Response<crate::generated::SubscriptionSchedule> {
    client.post_form(
        &format!("/subscription_schedules/{schedule}/cancel", schedule = schedule),
        params,
    )
}

pub fn post_subscription_schedules_schedule_release(
    client: &crate::Client,
    schedule: String,
    params: PostSubscriptionSchedulesScheduleReleaseParams,
) -> crate::Response<crate::generated::SubscriptionSchedule> {
    client.post_form(
        &format!("/subscription_schedules/{schedule}/release", schedule = schedule),
        params,
    )
}
