/// A coupon contains information about a percent-off or amount-off discount you
/// might want to apply to a customer.
///
/// Coupons may be applied to [subscriptions](https://stripe.com/docs/api#subscriptions), [invoices](https://stripe.com/docs/api#invoices), [checkout sessions](https://stripe.com/docs/api/checkout/sessions), [quotes](https://stripe.com/docs/api#quotes), and more.
/// Coupons do not work with conventional one-off [charges](https://stripe.com/docs/api#create_charge) or [payment intents](https://stripe.com/docs/api/payment_intents).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Coupon {
    /// Amount (in the `currency` specified) that will be taken off the subtotal of any invoices for this customer.
    pub amount_off: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<crate::generated::CouponAppliesTo>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// If `amount_off` has been set, the three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the amount to take off.
    pub currency: Option<crate::currency::Currency>,

    /// Coupons defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<crate::generated::CouponCurrencyOption>,

    /// One of `forever`, `once`, and `repeating`.
    ///
    /// Describes how long a customer who applies this coupon will get the discount.
    pub duration: CouponDuration,

    /// If `duration` is `repeating`, the number of months the coupon applies.
    ///
    /// Null if coupon `duration` is `forever` or `once`.
    pub duration_in_months: Option<i64>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Maximum number of times this coupon can be redeemed, in total, across all customers, before it is no longer valid.
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// Name of the coupon displayed to customers on for instance invoices or receipts.
    pub name: Option<String>,

    /// Percent that will be taken off the subtotal of any invoices for this customer for the duration of the coupon.
    ///
    /// For example, a coupon with percent_off of 50 will make a %s100 invoice %s50 instead.
    pub percent_off: Option<f64>,

    /// Date after which the coupon can no longer be redeemed.
    pub redeem_by: Option<crate::params::Timestamp>,

    /// Number of times this coupon has been applied to a customer.
    pub times_redeemed: i64,

    /// Taking account of the above properties, whether this coupon can still be applied to a customer.
    pub valid: bool,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCouponsParams {
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
pub struct PostCouponsParams {
    /// A positive integer representing the amount to subtract from an invoice total (required if `percent_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount_off: Option<i64>,

    /// A hash containing directions for what this Coupon will apply discounts to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub applies_to: Option<PostCouponsParamsAppliesTo>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) of the `amount_off` parameter (required if `amount_off` is passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// Coupons defined in each available currency option (only supported if `amount_off` is passed).
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<PostCouponsParamsCurrencyOptions>,

    /// Specifies how long the discount will be in effect if used on a subscription.
    ///
    /// Defaults to `once`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration: Option<PostCouponsParamsDuration>,

    /// Required only if `duration` is `repeating`, in which case it must be a positive integer that specifies the number of months the discount will be in effect.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub duration_in_months: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Unique string of your choice that will be used to identify this coupon when applying it to a customer.
    ///
    /// If you don't want to specify a particular code, you can leave the ID blank and we'll generate a random code for you.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,

    /// A positive integer specifying the number of times the coupon can be redeemed before it's no longer valid.
    ///
    /// For example, you might have a 50% off coupon that the first 20 readers of your blog can use.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// A positive float larger than 0, and smaller or equal to 100, that represents the discount the coupon will apply (required if `amount_off` is not passed).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percent_off: Option<f64>,

    /// Unix timestamp specifying the last time at which the coupon can be redeemed.
    ///
    /// After the redeem_by date, the coupon can no longer be applied to new customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redeem_by: Option<crate::params::Timestamp>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetCouponsCouponParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCouponsCouponParams {
    /// Coupons defined in each available currency option (only supported if the coupon is amount-based).
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<PostCouponsCouponParamsCurrencyOptions>,

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

    /// Name of the coupon displayed to customers on, for instance invoices, or receipts.
    ///
    /// By default the `id` is shown if `name` is not set.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CouponDuration {
    Forever,
    Once,
    Repeating,
}

impl CouponDuration {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forever => "forever",
            Self::Once => "once",
            Self::Repeating => "repeating",
        }
    }
}

impl AsRef<str> for CouponDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CouponDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CouponDuration {
    fn default() -> Self {
        Self::Forever
    }
}
/// A hash containing directions for what this Coupon will apply discounts to.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostCouponsParamsAppliesTo {
    /// An array of Product IDs that this Coupon will apply to.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub products: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCouponsParamsCurrencyOptions {
    /// A positive integer representing the amount to subtract from an invoice total.
    pub amount_off: i64,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostCouponsParamsDuration {
    Forever,
    Once,
    Repeating,
}

impl PostCouponsParamsDuration {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Forever => "forever",
            Self::Once => "once",
            Self::Repeating => "repeating",
        }
    }
}

impl AsRef<str> for PostCouponsParamsDuration {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostCouponsParamsDuration {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostCouponsParamsDuration {
    fn default() -> Self {
        Self::Forever
    }
}
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostCouponsCouponParamsCurrencyOptions {
    /// A positive integer representing the amount to subtract from an invoice total.
    pub amount_off: i64,
}

pub fn get_coupons(
    client: &crate::Client,
    params: GetCouponsParams,
) -> crate::Response<crate::params::List<crate::generated::Coupon>> {
    client.get_query("/coupons", params)
}

pub fn post_coupons(
    client: &crate::Client,
    params: PostCouponsParams,
) -> crate::Response<crate::generated::Coupon> {
    client.post_form("/coupons", params)
}

pub fn get_coupons_coupon(
    client: &crate::Client,
    coupon: String,
    params: GetCouponsCouponParams,
) -> crate::Response<crate::generated::Coupon> {
    client.get_query(&format!("/coupons/{coupon}", coupon = coupon), params)
}

pub fn post_coupons_coupon(
    client: &crate::Client,
    coupon: String,
    params: PostCouponsCouponParams,
) -> crate::Response<crate::generated::Coupon> {
    client.post_form(&format!("/coupons/{coupon}", coupon = coupon), params)
}

pub fn delete_coupons_coupon(
    client: &crate::Client,
    coupon: String,
) -> crate::Response<crate::generated::DeletedCoupon> {
    client.delete(&format!("/coupons/{coupon}", coupon = coupon))
}
