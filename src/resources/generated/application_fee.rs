#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ApplicationFee {
    /// ID of the Stripe account this fee was taken from.
    pub account: Vec<crate::generated::Account>,

    /// Amount earned, in %s.
    pub amount: i64,

    /// Amount in %s refunded (can be less than the amount attribute on the fee if a partial refund was issued).
    pub amount_refunded: i64,

    /// ID of the Connect application that earned the fee.
    pub application: Vec<crate::generated::Application>,

    /// Balance transaction that describes the impact of this collected application fee on your account balance (not including refunds).
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// ID of the charge that the application fee was taken from.
    pub charge: Vec<crate::generated::Charge>,

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

    /// ID of the corresponding charge on the platform account, if this fee was the result of a charge using the `destination` parameter.
    pub originating_transaction: Option<Vec<crate::generated::Charge>>,

    /// Whether the fee has been fully refunded.
    ///
    /// If the fee is only partially refunded, this attribute will still be false.
    pub refunded: bool,

    /// A list of refunds that have been applied to the fee.
    pub refunds: crate::params::List<crate::generated::FeeRefund>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetApplicationFeesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

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
pub struct GetApplicationFeesIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_application_fees(
    client: &crate::Client,
    params: GetApplicationFeesParams,
) -> crate::Response<crate::params::List<crate::generated::ApplicationFee>> {
    client.get_query("/application_fees", params)
}

pub fn get_application_fees_id(
    client: &crate::Client,
    id: String,
    params: GetApplicationFeesIdParams,
) -> crate::Response<crate::generated::ApplicationFee> {
    client.get_query(&format!("/application_fees/{id}", id = id), params)
}
