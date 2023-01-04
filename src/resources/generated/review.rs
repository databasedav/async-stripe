/// Reviews can be used to supplement automated fraud detection with human expertise.
///
/// Learn more about [Radar](/radar) and reviewing payments
/// [here](https://stripe.com/docs/radar/reviews).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Review {
    /// The ZIP or postal code of the card used, if applicable.
    pub billing_zip: Option<String>,

    /// The charge associated with this review.
    pub charge: Option<Vec<crate::generated::Charge>>,

    /// The reason the review was closed, or null if it has not yet been closed.
    ///
    /// One of `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub closed_reason: Option<ReviewClosedReason>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Unique identifier for the object.
    pub id: String,

    /// The IP address where the payment originated.
    pub ip_address: Option<String>,

    /// Information related to the location of the payment.
    ///
    /// Note that this information is an approximation and attempts to locate the nearest population center - it should not be used to determine a specific address.
    pub ip_address_location: Option<crate::generated::RadarReviewResourceLocation>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// If `true`, the review needs action.
    pub open: bool,

    /// The reason the review was opened.
    ///
    /// One of `rule` or `manual`.
    pub opened_reason: ReviewOpenedReason,

    /// The PaymentIntent ID associated with this review, if one exists.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,

    /// The reason the review is currently open or closed.
    ///
    /// One of `rule`, `manual`, `approved`, `refunded`, `refunded_as_fraud`, `disputed`, or `redacted`.
    pub reason: String,

    /// Information related to the browsing session of the user who initiated the payment.
    pub session: Option<crate::generated::RadarReviewResourceSession>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetReviewsParams {
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
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetReviewsReviewParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostReviewsReviewApproveParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewClosedReason {
    Approved,
    Disputed,
    Redacted,
    Refunded,
    RefundedAsFraud,
}

impl ReviewClosedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Approved => "approved",
            Self::Disputed => "disputed",
            Self::Redacted => "redacted",
            Self::Refunded => "refunded",
            Self::RefundedAsFraud => "refunded_as_fraud",
        }
    }
}

impl AsRef<str> for ReviewClosedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewClosedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ReviewClosedReason {
    fn default() -> Self {
        Self::Approved
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ReviewOpenedReason {
    Manual,
    Rule,
}

impl ReviewOpenedReason {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Manual => "manual",
            Self::Rule => "rule",
        }
    }
}

impl AsRef<str> for ReviewOpenedReason {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ReviewOpenedReason {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ReviewOpenedReason {
    fn default() -> Self {
        Self::Manual
    }
}
pub fn get_reviews(
    client: &crate::Client,
    params: GetReviewsParams,
) -> crate::Response<crate::params::List<crate::generated::Review>> {
    client.get_query("/reviews", params)
}

pub fn get_reviews_review(
    client: &crate::Client,
    review: String,
    params: GetReviewsReviewParams,
) -> crate::Response<crate::generated::Review> {
    client.get_query(&format!("/reviews/{review}", review = review), params)
}

pub fn post_reviews_review_approve(
    client: &crate::Client,
    review: String,
    params: PostReviewsReviewApproveParams,
) -> crate::Response<crate::generated::Review> {
    client.post_form(&format!("/reviews/{review}/approve", review = review), params)
}
