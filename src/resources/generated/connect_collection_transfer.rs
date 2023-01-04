#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ConnectCollectionTransfer {
    /// Amount transferred, in %s.
    pub amount: i64,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// ID of the account that funds are being collected for.
    pub destination: Vec<crate::generated::Account>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}
