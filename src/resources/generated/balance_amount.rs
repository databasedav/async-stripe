#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BalanceAmount {
    /// Balance amount.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_types: Option<crate::generated::BalanceAmountBySourceType>,
}
