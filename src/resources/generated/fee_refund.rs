/// `Application Fee Refund` objects allow you to refund an application fee that
/// has previously been created but not yet refunded.
///
/// Funds will be refunded to the Stripe account from which the fee was originally collected.  Related guide: [Refunding Application Fees](https://stripe.com/docs/connect/destination-charges#refunding-app-fee).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct FeeRefund {
    /// Amount, in %s.
    pub amount: i64,

    /// Balance transaction that describes the impact on your account balance.
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the application fee that was refunded.
    pub fee: Vec<crate::generated::ApplicationFee>,

    /// Unique identifier for the object.
    pub id: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostApplicationFeesIdRefundsParams {
    /// A positive integer, in _cents (or local equivalent)_, representing how much of this fee to refund.
    ///
    /// Can refund only up to the remaining unrefunded amount of the fee.
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
pub struct GetApplicationFeesIdRefundsParams {
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
pub struct GetApplicationFeesFeeRefundsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostApplicationFeesFeeRefundsIdParams {
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

pub fn post_application_fees_id_refunds(
    client: &crate::Client,
    id: String,
    params: PostApplicationFeesIdRefundsParams,
) -> crate::Response<crate::generated::FeeRefund> {
    client.post_form(&format!("/application_fees/{id}/refunds", id = id), params)
}

pub fn get_application_fees_id_refunds(
    client: &crate::Client,
    id: String,
    params: GetApplicationFeesIdRefundsParams,
) -> crate::Response<crate::params::List<crate::generated::FeeRefund>> {
    client.get_query(&format!("/application_fees/{id}/refunds", id = id), params)
}

pub fn get_application_fees_fee_refunds_id(
    client: &crate::Client,
    fee: String,
    id: String,
    params: GetApplicationFeesFeeRefundsIdParams,
) -> crate::Response<crate::generated::FeeRefund> {
    client.get_query(&format!("/application_fees/{fee}/refunds/{id}", fee = fee, id = id), params)
}

pub fn post_application_fees_fee_refunds_id(
    client: &crate::Client,
    fee: String,
    id: String,
    params: PostApplicationFeesFeeRefundsIdParams,
) -> crate::Response<crate::generated::FeeRefund> {
    client.post_form(&format!("/application_fees/{fee}/refunds/{id}", fee = fee, id = id), params)
}
