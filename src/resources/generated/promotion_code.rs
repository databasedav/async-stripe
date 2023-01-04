/// A Promotion Code represents a customer-redeemable code for a [coupon](https://stripe.com/docs/api#coupons).
///
/// It can be used to create multiple codes for a single coupon.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PromotionCode {
    /// Whether the promotion code is currently active.
    ///
    /// A promotion code is only active if the coupon is also valid.
    pub active: bool,

    /// The customer-facing code.
    ///
    /// Regardless of case, this code must be unique across all active promotion codes for each customer.
    pub code: String,

    pub coupon: crate::generated::Coupon,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The customer that this promotion code can be used by.
    pub customer: Option<Vec<crate::generated::Customer>>,

    /// Date at which the promotion code can no longer be redeemed.
    pub expires_at: Option<crate::resources::Scheduled>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Maximum number of times this promotion code can be redeemed.
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    pub restrictions: crate::generated::PromotionCodesResourceRestrictions,

    /// Number of times this promotion code has been used.
    pub times_redeemed: i64,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetPromotionCodesPromotionCodeParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesParams {
    /// Whether the promotion code is currently active.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    /// The customer-facing code.
    ///
    /// Regardless of case, this code must be unique across all active promotion codes for a specific customer.
    /// If left blank, we will generate one automatically.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    /// The coupon for this promotion code.
    pub coupon: String,

    /// The customer that this promotion code can be used by.
    ///
    /// If not set, the promotion code can be used by all customers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The timestamp at which this promotion code will expire.
    ///
    /// If the coupon has specified a `redeems_by`, then this value cannot be after the coupon's `redeems_by`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A positive integer specifying the number of times the promotion code can be redeemed.
    ///
    /// If the coupon has specified a `max_redemptions`, then this value cannot be greater than the coupon's `max_redemptions`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_redemptions: Option<i64>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Settings that restrict the redemption of the promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<PostPromotionCodesParamsRestrictions>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesPromotionCodeParams {
    /// Whether the promotion code is currently active.
    ///
    /// A promotion code can only be reactivated when the coupon is still valid and the promotion code is otherwise redeemable.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

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

    /// Settings that restrict the redemption of the promotion code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restrictions: Option<PostPromotionCodesPromotionCodeParamsRestrictions>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetPromotionCodesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub coupon: Option<String>,

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
    pub starting_after: Option<String>,
}

/// Settings that restrict the redemption of the promotion code.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesParamsRestrictions {
    /// Promotion codes defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<PostPromotionCodesParamsRestrictionsCurrencyOptions>,

    /// A Boolean indicating if the Promotion Code should only be redeemed for Customers without any successful payments or invoices.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_time_transaction: Option<bool>,

    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,

    /// Three-letter [ISO code](https://stripe.com/docs/currencies) for minimum_amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount_currency: Option<crate::currency::Currency>,
}

/// Settings that restrict the redemption of the promotion code.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesPromotionCodeParamsRestrictions {
    /// Promotion codes defined in each available currency option.
    ///
    /// Each key must be a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html) and a [supported currency](https://stripe.com/docs/currencies).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency_options: Option<PostPromotionCodesPromotionCodeParamsRestrictionsCurrencyOptions>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesParamsRestrictionsCurrencyOptions {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPromotionCodesPromotionCodeParamsRestrictionsCurrencyOptions {
    /// Minimum amount required to redeem this Promotion Code into a Coupon (e.g., a purchase must be $100 or more to work).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub minimum_amount: Option<i64>,
}

pub fn get_promotion_codes_promotion_code(
    client: &crate::Client,
    promotion_code: String,
    params: GetPromotionCodesPromotionCodeParams,
) -> crate::Response<crate::generated::PromotionCode> {
    client.get_query(
        &format!("/promotion_codes/{promotion_code}", promotion_code = promotion_code),
        params,
    )
}

pub fn post_promotion_codes(
    client: &crate::Client,
    params: PostPromotionCodesParams,
) -> crate::Response<crate::generated::PromotionCode> {
    client.post_form("/promotion_codes", params)
}

pub fn post_promotion_codes_promotion_code(
    client: &crate::Client,
    promotion_code: String,
    params: PostPromotionCodesPromotionCodeParams,
) -> crate::Response<crate::generated::PromotionCode> {
    client.post_form(
        &format!("/promotion_codes/{promotion_code}", promotion_code = promotion_code),
        params,
    )
}

pub fn get_promotion_codes(
    client: &crate::Client,
    params: GetPromotionCodesParams,
) -> crate::Response<crate::params::List<crate::generated::PromotionCode>> {
    client.get_query("/promotion_codes", params)
}
