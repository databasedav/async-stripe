/// You can [create physical or virtual cards](https://stripe.com/docs/issuing/cards) that are issued to cardholders.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCard {
    /// The brand of the card.
    pub brand: String,

    /// The reason why the card was canceled.
    pub cancellation_reason: Option<IssuingCardCancellationReason>,

    pub cardholder: crate::generated::IssuingCardholder,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Supported currencies are `usd` in the US, `eur` in the EU, and `gbp` in the UK.
    pub currency: crate::currency::Currency,

    /// The card's CVC.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,

    /// The expiration month of the card.
    pub exp_month: i64,

    /// The expiration year of the card.
    pub exp_year: i64,

    /// The financial account this card is attached to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// The last 4 digits of the card number.
    pub last4: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The full unredacted card number.
    ///
    /// For security reasons, this is only available for virtual cards, and will be omitted unless you explicitly request it with [the `expand` parameter](https://stripe.com/docs/api/expanding_objects).
    /// Additionally, it's only available via the ["Retrieve a card" endpoint](https://stripe.com/docs/api/issuing/cards/retrieve), not via "List all cards" or any other endpoint.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub number: Option<String>,

    /// The latest card that replaces this card, if any.
    pub replaced_by: Option<Vec<crate::generated::IssuingCard>>,

    /// The card this card replaces, if any.
    pub replacement_for: Option<Vec<crate::generated::IssuingCard>>,

    /// The reason why the previous card needed to be replaced.
    pub replacement_reason: Option<IssuingCardReplacementReason>,

    /// Where and how the card will be shipped.
    pub shipping: Option<crate::generated::IssuingCardShipping>,

    pub spending_controls: crate::generated::IssuingCardAuthorizationControls,

    /// Whether authorizations can be approved on this card.
    pub status: IssuingCardStatus,

    /// The type of the card.
    #[serde(rename = "type")]
    pub type_: IssuingCardType,

    /// Information relating to digital wallets (like Apple Pay and Google Pay).
    pub wallets: Option<crate::generated::IssuingCardWallets>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingCardsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub last4: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetIssuingCardsParamsStatus>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<GetIssuingCardsParamsType>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParams {
    /// The [Cardholder](https://stripe.com/docs/api#issuing_cardholder_object) object with which the card will be associated.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<String>,

    /// The currency for the card.
    pub currency: crate::currency::Currency,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_account: Option<String>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The card this is meant to be a replacement for (if any).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_for: Option<String>,

    /// If `replacement_for` is specified, this should indicate why that card is being replaced.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub replacement_reason: Option<PostIssuingCardsParamsReplacementReason>,

    /// The address where the card will be shipped.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostIssuingCardsParamsShipping>,

    /// Rules that control spending for this card.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<PostIssuingCardsParamsSpendingControls>,

    /// Whether authorizations can be approved on this card.
    ///
    /// Defaults to `inactive`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostIssuingCardsParamsStatus>,

    /// The type of card to issue.
    ///
    /// Possible values are `physical` or `virtual`.
    #[serde(rename = "type")]
    pub type_: PostIssuingCardsParamsType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingCardsCardParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsCardParams {
    /// Reason why the `status` of this card is `canceled`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<PostIssuingCardsCardParamsCancellationReason>,

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

    /// The desired new PIN for this card.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pin: Option<PostIssuingCardsCardParamsPin>,

    /// Rules that control spending for this card.
    ///
    /// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_controls: Option<PostIssuingCardsCardParamsSpendingControls>,

    /// Dictates whether authorizations can be approved on this card.
    ///
    /// If this card is being canceled because it was lost or stolen, this information should be provided as `cancellation_reason`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<PostIssuingCardsCardParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersIssuingCardsCardShippingDeliverParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersIssuingCardsCardShippingShipParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersIssuingCardsCardShippingReturnParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTestHelpersIssuingCardsCardShippingFailParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardCancellationReason {
    DesignRejected,
    Lost,
    Stolen,
}

impl IssuingCardCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DesignRejected => "design_rejected",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for IssuingCardCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardCancellationReason {
    fn default() -> Self {
        Self::DesignRejected
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl IssuingCardReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Damaged => "damaged",
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for IssuingCardReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardReplacementReason {
    fn default() -> Self {
        Self::Damaged
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardStatus {
    Active,
    Canceled,
    Inactive,
}

impl IssuingCardStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for IssuingCardStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardType {
    Physical,
    Virtual,
}

impl IssuingCardType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
        }
    }
}

impl AsRef<str> for IssuingCardType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardType {
    fn default() -> Self {
        Self::Physical
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIssuingCardsParamsStatus {
    Active,
    Canceled,
    Inactive,
}

impl GetIssuingCardsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for GetIssuingCardsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIssuingCardsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIssuingCardsParamsStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIssuingCardsParamsType {
    Physical,
    Virtual,
}

impl GetIssuingCardsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
        }
    }
}

impl AsRef<str> for GetIssuingCardsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIssuingCardsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIssuingCardsParamsType {
    fn default() -> Self {
        Self::Physical
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsReplacementReason {
    Damaged,
    Expired,
    Lost,
    Stolen,
}

impl PostIssuingCardsParamsReplacementReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Damaged => "damaged",
            Self::Expired => "expired",
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsReplacementReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsReplacementReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsReplacementReason {
    fn default() -> Self {
        Self::Damaged
    }
}
/// The address where the card will be shipped.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParamsShipping {
    /// The address that the card is shipped to.
    pub address: PostIssuingCardsParamsShippingAddress,

    /// Customs information for the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customs: Option<PostIssuingCardsParamsShippingCustoms>,

    /// The name printed on the shipping label when shipping the card.
    pub name: String,

    /// Phone number of the recipient of the shipment.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone_number: Option<String>,

    /// Whether a signature is required for card delivery.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_signature: Option<bool>,

    /// Shipment service.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service: Option<PostIssuingCardsParamsShippingService>,

    /// Packaging options.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostIssuingCardsParamsShippingType>,
}

/// Rules that control spending for this card.
///
/// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParamsSpendingControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories: Option<Vec<PostIssuingCardsParamsSpendingControlsAllowedCategories>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories: Option<Vec<PostIssuingCardsParamsSpendingControlsBlockedCategories>>,

    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<PostIssuingCardsParamsSpendingControlsSpendingLimits>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsStatus {
    Active,
    Inactive,
}

impl PostIssuingCardsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsType {
    Physical,
    Virtual,
}

impl PostIssuingCardsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Physical => "physical",
            Self::Virtual => "virtual",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsType {
    fn default() -> Self {
        Self::Physical
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsCancellationReason {
    Lost,
    Stolen,
}

impl PostIssuingCardsCardParamsCancellationReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lost => "lost",
            Self::Stolen => "stolen",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsCancellationReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsCancellationReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsCancellationReason {
    fn default() -> Self {
        Self::Lost
    }
}
/// The desired new PIN for this card.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsCardParamsPin {
    /// The card's desired new PIN, encrypted under Stripe's public key.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_number: Option<String>,
}

/// Rules that control spending for this card.
///
/// Refer to our [documentation](https://stripe.com/docs/issuing/controls/spending-controls) for more details.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsCardParamsSpendingControls {
    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to allow.
    ///
    /// All other categories will be blocked.
    /// Cannot be set with `blocked_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub allowed_categories:
        Option<Vec<PostIssuingCardsCardParamsSpendingControlsAllowedCategories>>,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) of authorizations to decline.
    ///
    /// All other categories will be allowed.
    /// Cannot be set with `allowed_categories`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blocked_categories:
        Option<Vec<PostIssuingCardsCardParamsSpendingControlsBlockedCategories>>,

    /// Limit spending with amount-based rules that apply across any cards this card replaced (i.e., its `replacement_for` card and _that_ card's `replacement_for` card, up the chain).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub spending_limits: Option<Vec<PostIssuingCardsCardParamsSpendingControlsSpendingLimits>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsStatus {
    Active,
    Canceled,
    Inactive,
}

impl PostIssuingCardsCardParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Canceled => "canceled",
            Self::Inactive => "inactive",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsStatus {
    fn default() -> Self {
        Self::Active
    }
}
/// The address that the card is shipped to.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParamsShippingAddress {
    /// City, district, suburb, town, or village.
    pub city: String,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: String,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    pub postal_code: String,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// Customs information for the shipment.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParamsShippingCustoms {
    /// The Economic Operators Registration and Identification (EORI) number to use for Customs.
    ///
    /// Required for bulk shipments to Europe.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eori_number: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsShippingService {
    Express,
    Priority,
    Standard,
}

impl PostIssuingCardsParamsShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Express => "express",
            Self::Priority => "priority",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsShippingService {
    fn default() -> Self {
        Self::Express
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsShippingType {
    Bulk,
    Individual,
}

impl PostIssuingCardsParamsShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bulk => "bulk",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsShippingType {
    fn default() -> Self {
        Self::Bulk
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsSpendingControlsAllowedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsParamsSpendingControlsAllowedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsSpendingControlsAllowedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsSpendingControlsAllowedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsSpendingControlsAllowedCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsSpendingControlsBlockedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsParamsSpendingControlsBlockedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsSpendingControlsBlockedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsSpendingControlsBlockedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsSpendingControlsBlockedCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsParamsSpendingControlsSpendingLimits {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories>>,

    /// Interval (or event) to which the amount applies.
    pub interval: PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsSpendingControlsAllowedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsCardParamsSpendingControlsAllowedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsSpendingControlsAllowedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsSpendingControlsAllowedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsSpendingControlsAllowedCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsSpendingControlsBlockedCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsCardParamsSpendingControlsBlockedCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsSpendingControlsBlockedCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsSpendingControlsBlockedCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsSpendingControlsBlockedCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingCardsCardParamsSpendingControlsSpendingLimits {
    /// Maximum amount allowed to spend per interval.
    pub amount: i64,

    /// Array of strings containing [categories](https://stripe.com/docs/api#issuing_authorization_object-merchant_data-category) this limit applies to.
    ///
    /// Omitting this field will apply the limit to all categories.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub categories: Option<Vec<PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories>>,

    /// Interval (or event) to which the amount applies.
    pub interval: PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsSpendingControlsSpendingLimitsCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllTime => "all_time",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::PerAuthorization => "per_authorization",
            Self::Weekly => "weekly",
            Self::Yearly => "yearly",
        }
    }
}

impl AsRef<str> for PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsParamsSpendingControlsSpendingLimitsInterval {
    fn default() -> Self {
        Self::AllTime
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories {
    AcRefrigerationRepair,
    AccountingBookkeepingServices,
    AdvertisingServices,
    AgriculturalCooperative,
    AirlinesAirCarriers,
    AirportsFlyingFields,
    AmbulanceServices,
    AmusementParksCarnivals,
    AntiqueReproductions,
    AntiqueShops,
    Aquariums,
    ArchitecturalSurveyingServices,
    ArtDealersAndGalleries,
    ArtistsSupplyAndCraftShops,
    AutoAndHomeSupplyStores,
    AutoBodyRepairShops,
    AutoPaintShops,
    AutoServiceShops,
    AutomatedCashDisburse,
    AutomatedFuelDispensers,
    AutomobileAssociations,
    AutomotivePartsAndAccessoriesStores,
    AutomotiveTireStores,
    BailAndBondPayments,
    Bakeries,
    BandsOrchestras,
    BarberAndBeautyShops,
    BettingCasinoGambling,
    BicycleShops,
    BilliardPoolEstablishments,
    BoatDealers,
    BoatRentalsAndLeases,
    BookStores,
    BooksPeriodicalsAndNewspapers,
    BowlingAlleys,
    BusLines,
    BusinessSecretarialSchools,
    BuyingShoppingServices,
    CableSatelliteAndOtherPayTelevisionAndRadio,
    CameraAndPhotographicSupplyStores,
    CandyNutAndConfectioneryStores,
    CarAndTruckDealersNewUsed,
    CarAndTruckDealersUsedOnly,
    CarRentalAgencies,
    CarWashes,
    CarpentryServices,
    CarpetUpholsteryCleaning,
    Caterers,
    CharitableAndSocialServiceOrganizationsFundraising,
    ChemicalsAndAlliedProducts,
    ChildCareServices,
    ChildrensAndInfantsWearStores,
    ChiropodistsPodiatrists,
    Chiropractors,
    CigarStoresAndStands,
    CivicSocialFraternalAssociations,
    CleaningAndMaintenance,
    ClothingRental,
    CollegesUniversities,
    CommercialEquipment,
    CommercialFootwear,
    CommercialPhotographyArtAndGraphics,
    CommuterTransportAndFerries,
    ComputerNetworkServices,
    ComputerProgramming,
    ComputerRepair,
    ComputerSoftwareStores,
    ComputersPeripheralsAndSoftware,
    ConcreteWorkServices,
    ConstructionMaterials,
    ConsultingPublicRelations,
    CorrespondenceSchools,
    CosmeticStores,
    CounselingServices,
    CountryClubs,
    CourierServices,
    CourtCosts,
    CreditReportingAgencies,
    CruiseLines,
    DairyProductsStores,
    DanceHallStudiosSchools,
    DatingEscortServices,
    DentistsOrthodontists,
    DepartmentStores,
    DetectiveAgencies,
    DigitalGoodsApplications,
    DigitalGoodsGames,
    DigitalGoodsLargeVolume,
    DigitalGoodsMedia,
    DirectMarketingCatalogMerchant,
    DirectMarketingCombinationCatalogAndRetailMerchant,
    DirectMarketingInboundTelemarketing,
    DirectMarketingInsuranceServices,
    DirectMarketingOther,
    DirectMarketingOutboundTelemarketing,
    DirectMarketingSubscription,
    DirectMarketingTravel,
    DiscountStores,
    Doctors,
    DoorToDoorSales,
    DraperyWindowCoveringAndUpholsteryStores,
    DrinkingPlaces,
    DrugStoresAndPharmacies,
    DrugsDrugProprietariesAndDruggistSundries,
    DryCleaners,
    DurableGoods,
    DutyFreeStores,
    EatingPlacesRestaurants,
    EducationalServices,
    ElectricRazorStores,
    ElectricalPartsAndEquipment,
    ElectricalServices,
    ElectronicsRepairShops,
    ElectronicsStores,
    ElementarySecondarySchools,
    EmploymentTempAgencies,
    EquipmentRental,
    ExterminatingServices,
    FamilyClothingStores,
    FastFoodRestaurants,
    FinancialInstitutions,
    FinesGovernmentAdministrativeEntities,
    FireplaceFireplaceScreensAndAccessoriesStores,
    FloorCoveringStores,
    Florists,
    FloristsSuppliesNurseryStockAndFlowers,
    FreezerAndLockerMeatProvisioners,
    FuelDealersNonAutomotive,
    FuneralServicesCrematories,
    FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances,
    FurnitureRepairRefinishing,
    FurriersAndFurShops,
    GeneralServices,
    GiftCardNoveltyAndSouvenirShops,
    GlassPaintAndWallpaperStores,
    GlasswareCrystalStores,
    GolfCoursesPublic,
    GovernmentServices,
    GroceryStoresSupermarkets,
    HardwareEquipmentAndSupplies,
    HardwareStores,
    HealthAndBeautySpas,
    HearingAidsSalesAndSupplies,
    #[serde(rename = "heating_plumbing_a_c")]
    HeatingPlumbingAC,
    HobbyToyAndGameShops,
    HomeSupplyWarehouseStores,
    Hospitals,
    HotelsMotelsAndResorts,
    HouseholdApplianceStores,
    IndustrialSupplies,
    InformationRetrievalServices,
    InsuranceDefault,
    InsuranceUnderwritingPremiums,
    IntraCompanyPurchases,
    JewelryStoresWatchesClocksAndSilverwareStores,
    LandscapingServices,
    Laundries,
    LaundryCleaningServices,
    LegalServicesAttorneys,
    LuggageAndLeatherGoodsStores,
    LumberBuildingMaterialsStores,
    ManualCashDisburse,
    MarinasServiceAndSupplies,
    MasonryStoneworkAndPlaster,
    MassageParlors,
    MedicalAndDentalLabs,
    MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies,
    MedicalServices,
    MembershipOrganizations,
    MensAndBoysClothingAndAccessoriesStores,
    MensWomensClothingStores,
    MetalServiceCenters,
    Miscellaneous,
    MiscellaneousApparelAndAccessoryShops,
    MiscellaneousAutoDealers,
    MiscellaneousBusinessServices,
    MiscellaneousFoodStores,
    MiscellaneousGeneralMerchandise,
    MiscellaneousGeneralServices,
    MiscellaneousHomeFurnishingSpecialtyStores,
    MiscellaneousPublishingAndPrinting,
    MiscellaneousRecreationServices,
    MiscellaneousRepairShops,
    MiscellaneousSpecialtyRetail,
    MobileHomeDealers,
    MotionPictureTheaters,
    MotorFreightCarriersAndTrucking,
    MotorHomesDealers,
    MotorVehicleSuppliesAndNewParts,
    MotorcycleShopsAndDealers,
    MotorcycleShopsDealers,
    MusicStoresMusicalInstrumentsPianosAndSheetMusic,
    NewsDealersAndNewsstands,
    NonFiMoneyOrders,
    NonFiStoredValueCardPurchaseLoad,
    NondurableGoods,
    NurseriesLawnAndGardenSupplyStores,
    NursingPersonalCare,
    OfficeAndCommercialFurniture,
    OpticiansEyeglasses,
    OptometristsOphthalmologist,
    OrthopedicGoodsProstheticDevices,
    Osteopaths,
    PackageStoresBeerWineAndLiquor,
    PaintsVarnishesAndSupplies,
    ParkingLotsGarages,
    PassengerRailways,
    PawnShops,
    PetShopsPetFoodAndSupplies,
    PetroleumAndPetroleumProducts,
    PhotoDeveloping,
    PhotographicPhotocopyMicrofilmEquipmentAndSupplies,
    PhotographicStudios,
    PictureVideoProduction,
    PieceGoodsNotionsAndOtherDryGoods,
    PlumbingHeatingEquipmentAndSupplies,
    PoliticalOrganizations,
    PostalServicesGovernmentOnly,
    PreciousStonesAndMetalsWatchesAndJewelry,
    ProfessionalServices,
    PublicWarehousingAndStorage,
    QuickCopyReproAndBlueprint,
    Railroads,
    RealEstateAgentsAndManagersRentals,
    RecordStores,
    RecreationalVehicleRentals,
    ReligiousGoodsStores,
    ReligiousOrganizations,
    RoofingSidingSheetMetal,
    SecretarialSupportServices,
    SecurityBrokersDealers,
    ServiceStations,
    SewingNeedleworkFabricAndPieceGoodsStores,
    ShoeRepairHatCleaning,
    ShoeStores,
    SmallApplianceRepair,
    SnowmobileDealers,
    SpecialTradeServices,
    SpecialtyCleaning,
    SportingGoodsStores,
    SportingRecreationCamps,
    SportsAndRidingApparelStores,
    SportsClubsFields,
    StampAndCoinStores,
    StationaryOfficeSuppliesPrintingAndWritingPaper,
    StationeryStoresOfficeAndSchoolSupplyStores,
    SwimmingPoolsSales,
    TUiTravelGermany,
    TailorsAlterations,
    TaxPaymentsGovernmentAgencies,
    TaxPreparationServices,
    TaxicabsLimousines,
    TelecommunicationEquipmentAndTelephoneSales,
    TelecommunicationServices,
    TelegraphServices,
    TentAndAwningShops,
    TestingLaboratories,
    TheatricalTicketAgencies,
    Timeshares,
    TireRetreadingAndRepair,
    TollsBridgeFees,
    TouristAttractionsAndExhibits,
    TowingServices,
    TrailerParksCampgrounds,
    TransportationServices,
    TravelAgenciesTourOperators,
    TruckStopIteration,
    TruckUtilityTrailerRentals,
    TypesettingPlateMakingAndRelatedServices,
    TypewriterStores,
    #[serde(rename = "u_s_federal_government_agencies_or_departments")]
    USFederalGovernmentAgenciesOrDepartments,
    UniformsCommercialClothing,
    UsedMerchandiseAndSecondhandStores,
    Utilities,
    VarietyStores,
    VeterinaryServices,
    VideoAmusementGameSupplies,
    VideoGameArcades,
    VideoTapeRentalStores,
    VocationalTradeSchools,
    WatchJewelryRepair,
    WeldingRepair,
    WholesaleClubs,
    WigAndToupeeStores,
    WiresMoneyOrders,
    WomensAccessoryAndSpecialtyShops,
    WomensReadyToWearStores,
    WreckingAndSalvageYards,
}

impl PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcRefrigerationRepair => "ac_refrigeration_repair",
            Self::AccountingBookkeepingServices => "accounting_bookkeeping_services",
            Self::AdvertisingServices => "advertising_services",
            Self::AgriculturalCooperative => "agricultural_cooperative",
            Self::AirlinesAirCarriers => "airlines_air_carriers",
            Self::AirportsFlyingFields => "airports_flying_fields",
            Self::AmbulanceServices => "ambulance_services",
            Self::AmusementParksCarnivals => "amusement_parks_carnivals",
            Self::AntiqueReproductions => "antique_reproductions",
            Self::AntiqueShops => "antique_shops",
            Self::Aquariums => "aquariums",
            Self::ArchitecturalSurveyingServices => "architectural_surveying_services",
            Self::ArtDealersAndGalleries => "art_dealers_and_galleries",
            Self::ArtistsSupplyAndCraftShops => "artists_supply_and_craft_shops",
            Self::AutoAndHomeSupplyStores => "auto_and_home_supply_stores",
            Self::AutoBodyRepairShops => "auto_body_repair_shops",
            Self::AutoPaintShops => "auto_paint_shops",
            Self::AutoServiceShops => "auto_service_shops",
            Self::AutomatedCashDisburse => "automated_cash_disburse",
            Self::AutomatedFuelDispensers => "automated_fuel_dispensers",
            Self::AutomobileAssociations => "automobile_associations",
            Self::AutomotivePartsAndAccessoriesStores => "automotive_parts_and_accessories_stores",
            Self::AutomotiveTireStores => "automotive_tire_stores",
            Self::BailAndBondPayments => "bail_and_bond_payments",
            Self::Bakeries => "bakeries",
            Self::BandsOrchestras => "bands_orchestras",
            Self::BarberAndBeautyShops => "barber_and_beauty_shops",
            Self::BettingCasinoGambling => "betting_casino_gambling",
            Self::BicycleShops => "bicycle_shops",
            Self::BilliardPoolEstablishments => "billiard_pool_establishments",
            Self::BoatDealers => "boat_dealers",
            Self::BoatRentalsAndLeases => "boat_rentals_and_leases",
            Self::BookStores => "book_stores",
            Self::BooksPeriodicalsAndNewspapers => "books_periodicals_and_newspapers",
            Self::BowlingAlleys => "bowling_alleys",
            Self::BusLines => "bus_lines",
            Self::BusinessSecretarialSchools => "business_secretarial_schools",
            Self::BuyingShoppingServices => "buying_shopping_services",
            Self::CableSatelliteAndOtherPayTelevisionAndRadio => {
                "cable_satellite_and_other_pay_television_and_radio"
            }
            Self::CameraAndPhotographicSupplyStores => "camera_and_photographic_supply_stores",
            Self::CandyNutAndConfectioneryStores => "candy_nut_and_confectionery_stores",
            Self::CarAndTruckDealersNewUsed => "car_and_truck_dealers_new_used",
            Self::CarAndTruckDealersUsedOnly => "car_and_truck_dealers_used_only",
            Self::CarRentalAgencies => "car_rental_agencies",
            Self::CarWashes => "car_washes",
            Self::CarpentryServices => "carpentry_services",
            Self::CarpetUpholsteryCleaning => "carpet_upholstery_cleaning",
            Self::Caterers => "caterers",
            Self::CharitableAndSocialServiceOrganizationsFundraising => {
                "charitable_and_social_service_organizations_fundraising"
            }
            Self::ChemicalsAndAlliedProducts => "chemicals_and_allied_products",
            Self::ChildCareServices => "child_care_services",
            Self::ChildrensAndInfantsWearStores => "childrens_and_infants_wear_stores",
            Self::ChiropodistsPodiatrists => "chiropodists_podiatrists",
            Self::Chiropractors => "chiropractors",
            Self::CigarStoresAndStands => "cigar_stores_and_stands",
            Self::CivicSocialFraternalAssociations => "civic_social_fraternal_associations",
            Self::CleaningAndMaintenance => "cleaning_and_maintenance",
            Self::ClothingRental => "clothing_rental",
            Self::CollegesUniversities => "colleges_universities",
            Self::CommercialEquipment => "commercial_equipment",
            Self::CommercialFootwear => "commercial_footwear",
            Self::CommercialPhotographyArtAndGraphics => "commercial_photography_art_and_graphics",
            Self::CommuterTransportAndFerries => "commuter_transport_and_ferries",
            Self::ComputerNetworkServices => "computer_network_services",
            Self::ComputerProgramming => "computer_programming",
            Self::ComputerRepair => "computer_repair",
            Self::ComputerSoftwareStores => "computer_software_stores",
            Self::ComputersPeripheralsAndSoftware => "computers_peripherals_and_software",
            Self::ConcreteWorkServices => "concrete_work_services",
            Self::ConstructionMaterials => "construction_materials",
            Self::ConsultingPublicRelations => "consulting_public_relations",
            Self::CorrespondenceSchools => "correspondence_schools",
            Self::CosmeticStores => "cosmetic_stores",
            Self::CounselingServices => "counseling_services",
            Self::CountryClubs => "country_clubs",
            Self::CourierServices => "courier_services",
            Self::CourtCosts => "court_costs",
            Self::CreditReportingAgencies => "credit_reporting_agencies",
            Self::CruiseLines => "cruise_lines",
            Self::DairyProductsStores => "dairy_products_stores",
            Self::DanceHallStudiosSchools => "dance_hall_studios_schools",
            Self::DatingEscortServices => "dating_escort_services",
            Self::DentistsOrthodontists => "dentists_orthodontists",
            Self::DepartmentStores => "department_stores",
            Self::DetectiveAgencies => "detective_agencies",
            Self::DigitalGoodsApplications => "digital_goods_applications",
            Self::DigitalGoodsGames => "digital_goods_games",
            Self::DigitalGoodsLargeVolume => "digital_goods_large_volume",
            Self::DigitalGoodsMedia => "digital_goods_media",
            Self::DirectMarketingCatalogMerchant => "direct_marketing_catalog_merchant",
            Self::DirectMarketingCombinationCatalogAndRetailMerchant => {
                "direct_marketing_combination_catalog_and_retail_merchant"
            }
            Self::DirectMarketingInboundTelemarketing => "direct_marketing_inbound_telemarketing",
            Self::DirectMarketingInsuranceServices => "direct_marketing_insurance_services",
            Self::DirectMarketingOther => "direct_marketing_other",
            Self::DirectMarketingOutboundTelemarketing => "direct_marketing_outbound_telemarketing",
            Self::DirectMarketingSubscription => "direct_marketing_subscription",
            Self::DirectMarketingTravel => "direct_marketing_travel",
            Self::DiscountStores => "discount_stores",
            Self::Doctors => "doctors",
            Self::DoorToDoorSales => "door_to_door_sales",
            Self::DraperyWindowCoveringAndUpholsteryStores => {
                "drapery_window_covering_and_upholstery_stores"
            }
            Self::DrinkingPlaces => "drinking_places",
            Self::DrugStoresAndPharmacies => "drug_stores_and_pharmacies",
            Self::DrugsDrugProprietariesAndDruggistSundries => {
                "drugs_drug_proprietaries_and_druggist_sundries"
            }
            Self::DryCleaners => "dry_cleaners",
            Self::DurableGoods => "durable_goods",
            Self::DutyFreeStores => "duty_free_stores",
            Self::EatingPlacesRestaurants => "eating_places_restaurants",
            Self::EducationalServices => "educational_services",
            Self::ElectricRazorStores => "electric_razor_stores",
            Self::ElectricalPartsAndEquipment => "electrical_parts_and_equipment",
            Self::ElectricalServices => "electrical_services",
            Self::ElectronicsRepairShops => "electronics_repair_shops",
            Self::ElectronicsStores => "electronics_stores",
            Self::ElementarySecondarySchools => "elementary_secondary_schools",
            Self::EmploymentTempAgencies => "employment_temp_agencies",
            Self::EquipmentRental => "equipment_rental",
            Self::ExterminatingServices => "exterminating_services",
            Self::FamilyClothingStores => "family_clothing_stores",
            Self::FastFoodRestaurants => "fast_food_restaurants",
            Self::FinancialInstitutions => "financial_institutions",
            Self::FinesGovernmentAdministrativeEntities => {
                "fines_government_administrative_entities"
            }
            Self::FireplaceFireplaceScreensAndAccessoriesStores => {
                "fireplace_fireplace_screens_and_accessories_stores"
            }
            Self::FloorCoveringStores => "floor_covering_stores",
            Self::Florists => "florists",
            Self::FloristsSuppliesNurseryStockAndFlowers => {
                "florists_supplies_nursery_stock_and_flowers"
            }
            Self::FreezerAndLockerMeatProvisioners => "freezer_and_locker_meat_provisioners",
            Self::FuelDealersNonAutomotive => "fuel_dealers_non_automotive",
            Self::FuneralServicesCrematories => "funeral_services_crematories",
            Self::FurnitureHomeFurnishingsAndEquipmentStoresExceptAppliances => {
                "furniture_home_furnishings_and_equipment_stores_except_appliances"
            }
            Self::FurnitureRepairRefinishing => "furniture_repair_refinishing",
            Self::FurriersAndFurShops => "furriers_and_fur_shops",
            Self::GeneralServices => "general_services",
            Self::GiftCardNoveltyAndSouvenirShops => "gift_card_novelty_and_souvenir_shops",
            Self::GlassPaintAndWallpaperStores => "glass_paint_and_wallpaper_stores",
            Self::GlasswareCrystalStores => "glassware_crystal_stores",
            Self::GolfCoursesPublic => "golf_courses_public",
            Self::GovernmentServices => "government_services",
            Self::GroceryStoresSupermarkets => "grocery_stores_supermarkets",
            Self::HardwareEquipmentAndSupplies => "hardware_equipment_and_supplies",
            Self::HardwareStores => "hardware_stores",
            Self::HealthAndBeautySpas => "health_and_beauty_spas",
            Self::HearingAidsSalesAndSupplies => "hearing_aids_sales_and_supplies",
            Self::HeatingPlumbingAC => "heating_plumbing_a_c",
            Self::HobbyToyAndGameShops => "hobby_toy_and_game_shops",
            Self::HomeSupplyWarehouseStores => "home_supply_warehouse_stores",
            Self::Hospitals => "hospitals",
            Self::HotelsMotelsAndResorts => "hotels_motels_and_resorts",
            Self::HouseholdApplianceStores => "household_appliance_stores",
            Self::IndustrialSupplies => "industrial_supplies",
            Self::InformationRetrievalServices => "information_retrieval_services",
            Self::InsuranceDefault => "insurance_default",
            Self::InsuranceUnderwritingPremiums => "insurance_underwriting_premiums",
            Self::IntraCompanyPurchases => "intra_company_purchases",
            Self::JewelryStoresWatchesClocksAndSilverwareStores => {
                "jewelry_stores_watches_clocks_and_silverware_stores"
            }
            Self::LandscapingServices => "landscaping_services",
            Self::Laundries => "laundries",
            Self::LaundryCleaningServices => "laundry_cleaning_services",
            Self::LegalServicesAttorneys => "legal_services_attorneys",
            Self::LuggageAndLeatherGoodsStores => "luggage_and_leather_goods_stores",
            Self::LumberBuildingMaterialsStores => "lumber_building_materials_stores",
            Self::ManualCashDisburse => "manual_cash_disburse",
            Self::MarinasServiceAndSupplies => "marinas_service_and_supplies",
            Self::MasonryStoneworkAndPlaster => "masonry_stonework_and_plaster",
            Self::MassageParlors => "massage_parlors",
            Self::MedicalAndDentalLabs => "medical_and_dental_labs",
            Self::MedicalDentalOphthalmicAndHospitalEquipmentAndSupplies => {
                "medical_dental_ophthalmic_and_hospital_equipment_and_supplies"
            }
            Self::MedicalServices => "medical_services",
            Self::MembershipOrganizations => "membership_organizations",
            Self::MensAndBoysClothingAndAccessoriesStores => {
                "mens_and_boys_clothing_and_accessories_stores"
            }
            Self::MensWomensClothingStores => "mens_womens_clothing_stores",
            Self::MetalServiceCenters => "metal_service_centers",
            Self::Miscellaneous => "miscellaneous",
            Self::MiscellaneousApparelAndAccessoryShops => {
                "miscellaneous_apparel_and_accessory_shops"
            }
            Self::MiscellaneousAutoDealers => "miscellaneous_auto_dealers",
            Self::MiscellaneousBusinessServices => "miscellaneous_business_services",
            Self::MiscellaneousFoodStores => "miscellaneous_food_stores",
            Self::MiscellaneousGeneralMerchandise => "miscellaneous_general_merchandise",
            Self::MiscellaneousGeneralServices => "miscellaneous_general_services",
            Self::MiscellaneousHomeFurnishingSpecialtyStores => {
                "miscellaneous_home_furnishing_specialty_stores"
            }
            Self::MiscellaneousPublishingAndPrinting => "miscellaneous_publishing_and_printing",
            Self::MiscellaneousRecreationServices => "miscellaneous_recreation_services",
            Self::MiscellaneousRepairShops => "miscellaneous_repair_shops",
            Self::MiscellaneousSpecialtyRetail => "miscellaneous_specialty_retail",
            Self::MobileHomeDealers => "mobile_home_dealers",
            Self::MotionPictureTheaters => "motion_picture_theaters",
            Self::MotorFreightCarriersAndTrucking => "motor_freight_carriers_and_trucking",
            Self::MotorHomesDealers => "motor_homes_dealers",
            Self::MotorVehicleSuppliesAndNewParts => "motor_vehicle_supplies_and_new_parts",
            Self::MotorcycleShopsAndDealers => "motorcycle_shops_and_dealers",
            Self::MotorcycleShopsDealers => "motorcycle_shops_dealers",
            Self::MusicStoresMusicalInstrumentsPianosAndSheetMusic => {
                "music_stores_musical_instruments_pianos_and_sheet_music"
            }
            Self::NewsDealersAndNewsstands => "news_dealers_and_newsstands",
            Self::NonFiMoneyOrders => "non_fi_money_orders",
            Self::NonFiStoredValueCardPurchaseLoad => "non_fi_stored_value_card_purchase_load",
            Self::NondurableGoods => "nondurable_goods",
            Self::NurseriesLawnAndGardenSupplyStores => "nurseries_lawn_and_garden_supply_stores",
            Self::NursingPersonalCare => "nursing_personal_care",
            Self::OfficeAndCommercialFurniture => "office_and_commercial_furniture",
            Self::OpticiansEyeglasses => "opticians_eyeglasses",
            Self::OptometristsOphthalmologist => "optometrists_ophthalmologist",
            Self::OrthopedicGoodsProstheticDevices => "orthopedic_goods_prosthetic_devices",
            Self::Osteopaths => "osteopaths",
            Self::PackageStoresBeerWineAndLiquor => "package_stores_beer_wine_and_liquor",
            Self::PaintsVarnishesAndSupplies => "paints_varnishes_and_supplies",
            Self::ParkingLotsGarages => "parking_lots_garages",
            Self::PassengerRailways => "passenger_railways",
            Self::PawnShops => "pawn_shops",
            Self::PetShopsPetFoodAndSupplies => "pet_shops_pet_food_and_supplies",
            Self::PetroleumAndPetroleumProducts => "petroleum_and_petroleum_products",
            Self::PhotoDeveloping => "photo_developing",
            Self::PhotographicPhotocopyMicrofilmEquipmentAndSupplies => {
                "photographic_photocopy_microfilm_equipment_and_supplies"
            }
            Self::PhotographicStudios => "photographic_studios",
            Self::PictureVideoProduction => "picture_video_production",
            Self::PieceGoodsNotionsAndOtherDryGoods => "piece_goods_notions_and_other_dry_goods",
            Self::PlumbingHeatingEquipmentAndSupplies => "plumbing_heating_equipment_and_supplies",
            Self::PoliticalOrganizations => "political_organizations",
            Self::PostalServicesGovernmentOnly => "postal_services_government_only",
            Self::PreciousStonesAndMetalsWatchesAndJewelry => {
                "precious_stones_and_metals_watches_and_jewelry"
            }
            Self::ProfessionalServices => "professional_services",
            Self::PublicWarehousingAndStorage => "public_warehousing_and_storage",
            Self::QuickCopyReproAndBlueprint => "quick_copy_repro_and_blueprint",
            Self::Railroads => "railroads",
            Self::RealEstateAgentsAndManagersRentals => "real_estate_agents_and_managers_rentals",
            Self::RecordStores => "record_stores",
            Self::RecreationalVehicleRentals => "recreational_vehicle_rentals",
            Self::ReligiousGoodsStores => "religious_goods_stores",
            Self::ReligiousOrganizations => "religious_organizations",
            Self::RoofingSidingSheetMetal => "roofing_siding_sheet_metal",
            Self::SecretarialSupportServices => "secretarial_support_services",
            Self::SecurityBrokersDealers => "security_brokers_dealers",
            Self::ServiceStations => "service_stations",
            Self::SewingNeedleworkFabricAndPieceGoodsStores => {
                "sewing_needlework_fabric_and_piece_goods_stores"
            }
            Self::ShoeRepairHatCleaning => "shoe_repair_hat_cleaning",
            Self::ShoeStores => "shoe_stores",
            Self::SmallApplianceRepair => "small_appliance_repair",
            Self::SnowmobileDealers => "snowmobile_dealers",
            Self::SpecialTradeServices => "special_trade_services",
            Self::SpecialtyCleaning => "specialty_cleaning",
            Self::SportingGoodsStores => "sporting_goods_stores",
            Self::SportingRecreationCamps => "sporting_recreation_camps",
            Self::SportsAndRidingApparelStores => "sports_and_riding_apparel_stores",
            Self::SportsClubsFields => "sports_clubs_fields",
            Self::StampAndCoinStores => "stamp_and_coin_stores",
            Self::StationaryOfficeSuppliesPrintingAndWritingPaper => {
                "stationary_office_supplies_printing_and_writing_paper"
            }
            Self::StationeryStoresOfficeAndSchoolSupplyStores => {
                "stationery_stores_office_and_school_supply_stores"
            }
            Self::SwimmingPoolsSales => "swimming_pools_sales",
            Self::TUiTravelGermany => "t_ui_travel_germany",
            Self::TailorsAlterations => "tailors_alterations",
            Self::TaxPaymentsGovernmentAgencies => "tax_payments_government_agencies",
            Self::TaxPreparationServices => "tax_preparation_services",
            Self::TaxicabsLimousines => "taxicabs_limousines",
            Self::TelecommunicationEquipmentAndTelephoneSales => {
                "telecommunication_equipment_and_telephone_sales"
            }
            Self::TelecommunicationServices => "telecommunication_services",
            Self::TelegraphServices => "telegraph_services",
            Self::TentAndAwningShops => "tent_and_awning_shops",
            Self::TestingLaboratories => "testing_laboratories",
            Self::TheatricalTicketAgencies => "theatrical_ticket_agencies",
            Self::Timeshares => "timeshares",
            Self::TireRetreadingAndRepair => "tire_retreading_and_repair",
            Self::TollsBridgeFees => "tolls_bridge_fees",
            Self::TouristAttractionsAndExhibits => "tourist_attractions_and_exhibits",
            Self::TowingServices => "towing_services",
            Self::TrailerParksCampgrounds => "trailer_parks_campgrounds",
            Self::TransportationServices => "transportation_services",
            Self::TravelAgenciesTourOperators => "travel_agencies_tour_operators",
            Self::TruckStopIteration => "truck_stop_iteration",
            Self::TruckUtilityTrailerRentals => "truck_utility_trailer_rentals",
            Self::TypesettingPlateMakingAndRelatedServices => {
                "typesetting_plate_making_and_related_services"
            }
            Self::TypewriterStores => "typewriter_stores",
            Self::USFederalGovernmentAgenciesOrDepartments => {
                "u_s_federal_government_agencies_or_departments"
            }
            Self::UniformsCommercialClothing => "uniforms_commercial_clothing",
            Self::UsedMerchandiseAndSecondhandStores => "used_merchandise_and_secondhand_stores",
            Self::Utilities => "utilities",
            Self::VarietyStores => "variety_stores",
            Self::VeterinaryServices => "veterinary_services",
            Self::VideoAmusementGameSupplies => "video_amusement_game_supplies",
            Self::VideoGameArcades => "video_game_arcades",
            Self::VideoTapeRentalStores => "video_tape_rental_stores",
            Self::VocationalTradeSchools => "vocational_trade_schools",
            Self::WatchJewelryRepair => "watch_jewelry_repair",
            Self::WeldingRepair => "welding_repair",
            Self::WholesaleClubs => "wholesale_clubs",
            Self::WigAndToupeeStores => "wig_and_toupee_stores",
            Self::WiresMoneyOrders => "wires_money_orders",
            Self::WomensAccessoryAndSpecialtyShops => "womens_accessory_and_specialty_shops",
            Self::WomensReadyToWearStores => "womens_ready_to_wear_stores",
            Self::WreckingAndSalvageYards => "wrecking_and_salvage_yards",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsCategories {
    fn default() -> Self {
        Self::AcRefrigerationRepair
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval {
    AllTime,
    Daily,
    Monthly,
    PerAuthorization,
    Weekly,
    Yearly,
}

impl PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AllTime => "all_time",
            Self::Daily => "daily",
            Self::Monthly => "monthly",
            Self::PerAuthorization => "per_authorization",
            Self::Weekly => "weekly",
            Self::Yearly => "yearly",
        }
    }
}

impl AsRef<str> for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostIssuingCardsCardParamsSpendingControlsSpendingLimitsInterval {
    fn default() -> Self {
        Self::AllTime
    }
}
pub fn get_issuing_cards(
    client: &crate::Client,
    params: GetIssuingCardsParams,
) -> crate::Response<crate::params::List<crate::generated::IssuingCard>> {
    client.get_query("/issuing/cards", params)
}

pub fn post_issuing_cards(
    client: &crate::Client,
    params: PostIssuingCardsParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form("/issuing/cards", params)
}

pub fn get_issuing_cards_card(
    client: &crate::Client,
    card: String,
    params: GetIssuingCardsCardParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.get_query(&format!("/issuing/cards/{card}", card = card), params)
}

pub fn post_issuing_cards_card(
    client: &crate::Client,
    card: String,
    params: PostIssuingCardsCardParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form(&format!("/issuing/cards/{card}", card = card), params)
}

pub fn post_test_helpers_issuing_cards_card_shipping_deliver(
    client: &crate::Client,
    card: String,
    params: PostTestHelpersIssuingCardsCardShippingDeliverParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form(
        &format!("/test_helpers/issuing/cards/{card}/shipping/deliver", card = card),
        params,
    )
}

pub fn post_test_helpers_issuing_cards_card_shipping_ship(
    client: &crate::Client,
    card: String,
    params: PostTestHelpersIssuingCardsCardShippingShipParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form(
        &format!("/test_helpers/issuing/cards/{card}/shipping/ship", card = card),
        params,
    )
}

pub fn post_test_helpers_issuing_cards_card_shipping_return(
    client: &crate::Client,
    card: String,
    params: PostTestHelpersIssuingCardsCardShippingReturnParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form(
        &format!("/test_helpers/issuing/cards/{card}/shipping/return", card = card),
        params,
    )
}

pub fn post_test_helpers_issuing_cards_card_shipping_fail(
    client: &crate::Client,
    card: String,
    params: PostTestHelpersIssuingCardsCardShippingFailParams,
) -> crate::Response<crate::generated::IssuingCard> {
    client.post_form(
        &format!("/test_helpers/issuing/cards/{card}/shipping/fail", card = card),
        params,
    )
}
