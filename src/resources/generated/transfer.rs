/// A `Transfer` object is created when you move funds between Stripe accounts as
/// part of Connect.
///
/// Before April 6, 2017, transfers also represented movement of funds from a
/// Stripe account to a card or bank account.
///
/// This behavior has since been split out into a [Payout](https://stripe.com/docs/api#payout_object) object, with corresponding payout endpoints.
/// For more information, read about the [transfer/payout split](https://stripe.com/docs/transfer-payout-split).  Related guide: [Creating Separate Charges and Transfers](https://stripe.com/docs/connect/charges-transfers).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Transfer {
    /// Amount in %s to be transferred.
    pub amount: i64,

    /// Amount in %s reversed (can be less than the amount attribute on the transfer if a partial reversal was issued).
    pub amount_reversed: i64,

    /// Balance transaction that describes the impact of this transfer on your account balance.
    pub balance_transaction: Option<Vec<crate::generated::BalanceTransaction>>,

    /// Time that this record of the transfer was first created.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    pub description: Option<String>,

    /// ID of the Stripe account the transfer was sent to.
    pub destination: Option<Vec<crate::generated::Account>>,

    /// If the destination is a Stripe account, this will be the ID of the payment that the destination account received for the transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination_payment: Option<Vec<crate::generated::Charge>>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// A list of reversals that have been applied to the transfer.
    pub reversals: crate::params::List<crate::generated::TransferReversal>,

    /// Whether the transfer has been fully reversed.
    ///
    /// If the transfer is only partially reversed, this attribute will still be false.
    pub reversed: bool,

    /// ID of the charge or payment that was used to fund the transfer.
    ///
    /// If null, the transfer was funded from the available balance.
    pub source_transaction: Option<Vec<crate::generated::Charge>>,

    /// The source balance this transfer came from.
    ///
    /// One of `card`, `fpx`, or `bank_account`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<String>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTransfersParams {
    /// A positive integer in cents (or local equivalent) representing how much to transfer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// 3-letter [ISO code for currency](https://stripe.com/docs/payouts).
    pub currency: crate::currency::Currency,

    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The ID of a connected Stripe account.
    ///
    /// [See the Connect documentation](https://stripe.com/docs/connect/charges-transfers) for details.
    pub destination: String,

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

    /// You can use this parameter to transfer funds from a charge before they are added to your available balance.
    ///
    /// A pending balance will transfer immediately but the funds will not become available until the original charge becomes available.
    /// [See the Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-availability) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_transaction: Option<String>,

    /// The source balance to use for this transfer.
    ///
    /// One of `bank_account`, `card`, or `fpx`.
    /// For most users, this will default to `card`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_type: Option<PostTransfersParamsSourceType>,

    /// A string that identifies this transaction as part of a group.
    ///
    /// See the [Connect documentation](https://stripe.com/docs/connect/charges-transfers#transfer-options) for details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTransfersParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created: Option<crate::params::RangeQueryTs>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub destination: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub transfer_group: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTransfersTransferParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTransfersTransferParams {
    /// An arbitrary string attached to the object.
    ///
    /// Often useful for displaying to users.
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
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTransfersParamsSourceType {
    BankAccount,
    Card,
    Fpx,
}

impl PostTransfersParamsSourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BankAccount => "bank_account",
            Self::Card => "card",
            Self::Fpx => "fpx",
        }
    }
}

impl AsRef<str> for PostTransfersParamsSourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTransfersParamsSourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTransfersParamsSourceType {
    fn default() -> Self {
        Self::BankAccount
    }
}
pub fn post_transfers(
    client: &crate::Client,
    params: PostTransfersParams,
) -> crate::Response<crate::generated::Transfer> {
    client.post_form("/transfers", params)
}

pub fn get_transfers(
    client: &crate::Client,
    params: GetTransfersParams,
) -> crate::Response<crate::params::List<crate::generated::Transfer>> {
    client.get_query("/transfers", params)
}

pub fn get_transfers_transfer(
    client: &crate::Client,
    transfer: String,
    params: GetTransfersTransferParams,
) -> crate::Response<crate::generated::Transfer> {
    client.get_query(&format!("/transfers/{transfer}", transfer = transfer), params)
}

pub fn post_transfers_transfer(
    client: &crate::Client,
    transfer: String,
    params: PostTransfersTransferParams,
) -> crate::Response<crate::generated::Transfer> {
    client.post_form(&format!("/transfers/{transfer}", transfer = transfer), params)
}
