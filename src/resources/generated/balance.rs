/// This is an object representing your Stripe balance.
///
/// You can retrieve it to see the balance currently on your Stripe account.  You can also retrieve the balance history, which contains a list of [transactions](https://stripe.com/docs/reporting/balance-transaction-types) that contributed to the balance (charges, payouts, and so forth).  The available and pending amounts for each currency are broken down further by payment source types.  Related guide: [Understanding Connect Account Balances](https://stripe.com/docs/connect/account-balances).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Balance {
    /// Funds that are available to be transferred or paid out, whether automatically by Stripe or explicitly via the [Transfers API](https://stripe.com/docs/api#transfers) or [Payouts API](https://stripe.com/docs/api#payouts).
    ///
    /// The available balance for each currency and payment type can be found in the `source_types` property.
    pub available: Vec<crate::generated::BalanceAmount>,

    /// Funds held due to negative balances on connected Custom accounts.
    ///
    /// The connect reserve balance for each currency and payment type can be found in the `source_types` property.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub connect_reserved: Option<Vec<crate::generated::BalanceAmount>>,

    /// Funds that can be paid out using Instant Payouts.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instant_available: Option<Vec<crate::generated::BalanceAmount>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub issuing: Option<crate::generated::BalanceDetail>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Funds that are not yet available in the balance, due to the 7-day rolling pay cycle.
    ///
    /// The pending balance for each currency, and for each payment type, can be found in the `source_types` property.
    pub pending: Vec<crate::generated::BalanceAmount>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetBalanceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_balance(
    client: &crate::Client,
    params: GetBalanceParams,
) -> crate::Response<crate::generated::Balance> {
    client.get_query("/balance", params)
}
