/// When an [issued card](https://stripe.com/docs/issuing) is used to make a purchase, an Issuing `Authorization`
/// object is created.
///
/// [Authorizations](https://stripe.com/docs/issuing/purchases/authorizations) must be approved for the purchase to be completed successfully.  Related guide: [Issued Card Authorizations](https://stripe.com/docs/issuing/purchases/authorizations).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingAuthorization {
    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the card's currency and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount: i64,

    /// Detailed breakdown of amount components.
    ///
    /// These amounts are denominated in `currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub amount_details: Option<crate::generated::IssuingAuthorizationAmountDetails>,

    /// Whether the authorization has been approved.
    pub approved: bool,

    /// How the card details were provided.
    pub authorization_method: IssuingAuthorizationAuthorizationMethod,

    /// List of balance transactions associated with this authorization.
    pub balance_transactions: Vec<crate::generated::BalanceTransaction>,

    pub card: crate::generated::IssuingCard,

    /// The cardholder to whom this authorization belongs.
    pub cardholder: Option<Vec<crate::generated::IssuingCardholder>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The total amount that was authorized or rejected.
    ///
    /// This amount is in the `merchant_currency` and in the [smallest currency unit](https://stripe.com/docs/currencies#zero-decimal).
    pub merchant_amount: i64,

    /// The currency that was presented to the cardholder for the authorization.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub merchant_currency: crate::currency::Currency,

    pub merchant_data: crate::generated::IssuingAuthorizationMerchantData,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// Details about the authorization, such as identifiers, set by the card network.
    pub network_data: Option<crate::generated::IssuingAuthorizationNetworkData>,

    /// The pending authorization request.
    ///
    /// This field will only be non-null during an `issuing_authorization.request` webhook.
    pub pending_request: Option<crate::generated::IssuingAuthorizationPendingRequest>,

    /// History of every time `pending_request` was approved/denied, either by you directly or by Stripe (e.g.
    ///
    /// based on your `spending_controls`).
    /// If the merchant changes the authorization by performing an [incremental authorization](https://stripe.com/docs/issuing/purchases/authorizations), you can look at this field to see the previous requests for the authorization.
    pub request_history: Vec<crate::generated::IssuingAuthorizationRequest>,

    /// The current status of the authorization in its lifecycle.
    pub status: IssuingAuthorizationStatus,

    /// List of [transactions](https://stripe.com/docs/api/issuing/transactions) associated with this authorization.
    pub transactions: Vec<crate::generated::IssuingTransaction>,

    /// [Treasury](https://stripe.com/docs/api/treasury) details related to this authorization if it was created on a [FinancialAccount](https://stripe.com/docs/api/treasury/financial_accounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub treasury: Option<crate::generated::IssuingAuthorizationTreasury>,

    pub verification_data: crate::generated::IssuingAuthorizationVerificationData,

    /// The digital wallet used for this authorization.
    ///
    /// One of `apple_pay`, `google_pay`, or `samsung_pay`.
    pub wallet: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingAuthorizationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cardholder: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetIssuingAuthorizationsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetIssuingAuthorizationsAuthorizationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingAuthorizationsAuthorizationParams {
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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingAuthorizationsAuthorizationApproveParams {
    /// If the authorization's `pending_request.is_amount_controllable` property is `true`, you may provide this value to control how much to hold for the authorization.
    ///
    /// Must be positive (use [`decline`](https://stripe.com/docs/api/issuing/authorizations/decline) to decline an authorization request).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostIssuingAuthorizationsAuthorizationDeclineParams {
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
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationAuthorizationMethod {
    Chip,
    Contactless,
    KeyedIn,
    Online,
    Swipe,
}

impl IssuingAuthorizationAuthorizationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Chip => "chip",
            Self::Contactless => "contactless",
            Self::KeyedIn => "keyed_in",
            Self::Online => "online",
            Self::Swipe => "swipe",
        }
    }
}

impl AsRef<str> for IssuingAuthorizationAuthorizationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingAuthorizationAuthorizationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingAuthorizationAuthorizationMethod {
    fn default() -> Self {
        Self::Chip
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingAuthorizationStatus {
    Closed,
    Pending,
    Reversed,
}

impl IssuingAuthorizationStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Pending => "pending",
            Self::Reversed => "reversed",
        }
    }
}

impl AsRef<str> for IssuingAuthorizationStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingAuthorizationStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingAuthorizationStatus {
    fn default() -> Self {
        Self::Closed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetIssuingAuthorizationsParamsStatus {
    Closed,
    Pending,
    Reversed,
}

impl GetIssuingAuthorizationsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Pending => "pending",
            Self::Reversed => "reversed",
        }
    }
}

impl AsRef<str> for GetIssuingAuthorizationsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetIssuingAuthorizationsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetIssuingAuthorizationsParamsStatus {
    fn default() -> Self {
        Self::Closed
    }
}
pub fn get_issuing_authorizations(
    client: &crate::Client,
    params: GetIssuingAuthorizationsParams,
) -> crate::Response<crate::params::List<crate::generated::IssuingAuthorization>> {
    client.get_query("/issuing/authorizations", params)
}

pub fn get_issuing_authorizations_authorization(
    client: &crate::Client,
    authorization: String,
    params: GetIssuingAuthorizationsAuthorizationParams,
) -> crate::Response<crate::generated::IssuingAuthorization> {
    client.get_query(
        &format!("/issuing/authorizations/{authorization}", authorization = authorization),
        params,
    )
}

pub fn post_issuing_authorizations_authorization(
    client: &crate::Client,
    authorization: String,
    params: PostIssuingAuthorizationsAuthorizationParams,
) -> crate::Response<crate::generated::IssuingAuthorization> {
    client.post_form(
        &format!("/issuing/authorizations/{authorization}", authorization = authorization),
        params,
    )
}

pub fn post_issuing_authorizations_authorization_approve(
    client: &crate::Client,
    authorization: String,
    params: PostIssuingAuthorizationsAuthorizationApproveParams,
) -> crate::Response<crate::generated::IssuingAuthorization> {
    client.post_form(
        &format!("/issuing/authorizations/{authorization}/approve", authorization = authorization),
        params,
    )
}

pub fn post_issuing_authorizations_authorization_decline(
    client: &crate::Client,
    authorization: String,
    params: PostIssuingAuthorizationsAuthorizationDeclineParams,
) -> crate::Response<crate::generated::IssuingAuthorization> {
    client.post_form(
        &format!("/issuing/authorizations/{authorization}/decline", authorization = authorization),
        params,
    )
}
