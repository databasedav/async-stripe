/// [Stripe Connect](https://stripe.com/docs/connect) platforms can reverse transfers made to a
/// connected account, either entirely or partially, and can also specify whether
/// to refund any related application fees.
///
/// Transfer reversals add to the platform's balance and subtract from the destination account's balance.  Reversing a transfer that was made for a [destination charge](/docs/connect/destination-charges) is allowed only up to the amount of the charge.
/// It is possible to reverse a [transfer_group](https://stripe.com/docs/connect/charges-transfers#transfer-options) transfer only if the destination account has enough balance to cover the reversal.  Related guide: [Reversing Transfers](https://stripe.com/docs/connect/charges-transfers#reversing-transfers).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TransferReversal {
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

    /// Linked payment refund for the transfer reversal.
    pub destination_payment_refund: Option<Vec<crate::generated::Refund>>,

    /// Unique identifier for the object.
    pub id: String,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// ID of the refund responsible for the transfer reversal.
    pub source_refund: Option<Vec<crate::generated::Refund>>,

    /// ID of the transfer that was reversed.
    pub transfer: Vec<crate::generated::Transfer>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTransfersIdReversalsParams {
    /// A positive integer in cents (or local equivalent) representing how much of this transfer to reverse.
    ///
    /// Can only reverse up to the unreversed amount remaining of the transfer.
    /// Partial transfer reversals are only allowed for transfers to Stripe Accounts.
    /// Defaults to the entire transfer amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// An arbitrary string which you can attach to a reversal object.
    ///
    /// It is displayed alongside the reversal in the Dashboard.
    /// This will be unset if you POST an empty value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

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

    /// Boolean indicating whether the application fee should be refunded when reversing this transfer.
    ///
    /// If a full transfer reversal is given, the full application fee will be refunded.
    /// Otherwise, the application fee will be refunded with an amount proportional to the amount of the transfer reversed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_application_fee: Option<bool>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTransfersIdReversalsParams {
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
pub struct GetTransfersTransferReversalsIdParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTransfersTransferReversalsIdParams {
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

pub fn post_transfers_id_reversals(
    client: &crate::Client,
    id: String,
    params: PostTransfersIdReversalsParams,
) -> crate::Response<crate::generated::TransferReversal> {
    client.post_form(&format!("/transfers/{id}/reversals", id = id), params)
}

pub fn get_transfers_id_reversals(
    client: &crate::Client,
    id: String,
    params: GetTransfersIdReversalsParams,
) -> crate::Response<crate::params::List<crate::generated::TransferReversal>> {
    client.get_query(&format!("/transfers/{id}/reversals", id = id), params)
}

pub fn get_transfers_transfer_reversals_id(
    client: &crate::Client,
    id: String,
    transfer: String,
    params: GetTransfersTransferReversalsIdParams,
) -> crate::Response<crate::generated::TransferReversal> {
    client.get_query(
        &format!("/transfers/{transfer}/reversals/{id}", id = id, transfer = transfer),
        params,
    )
}

pub fn post_transfers_transfer_reversals_id(
    client: &crate::Client,
    id: String,
    transfer: String,
    params: PostTransfersTransferReversalsIdParams,
) -> crate::Response<crate::generated::TransferReversal> {
    client.post_form(
        &format!("/transfers/{transfer}/reversals/{id}", id = id, transfer = transfer),
        params,
    )
}
