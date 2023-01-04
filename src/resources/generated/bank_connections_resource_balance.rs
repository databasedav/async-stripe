#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct BankConnectionsResourceBalance {
    /// The time that the external institution calculated this balance.
    ///
    /// Measured in seconds since the Unix epoch.
    pub as_of: crate::params::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cash: Option<crate::generated::BankConnectionsResourceBalanceApiResourceCashBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub credit: Option<crate::generated::BankConnectionsResourceBalanceApiResourceCreditBalance>,

    /// The balances owed to (or by) the account holder.
    ///
    /// Each key is a three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Each value is a integer amount.
    ///
    /// A positive amount indicates money owed to the account holder.
    /// A negative amount indicates money owed by the account holder.
    pub current: i64,

    /// The `type` of the balance.
    ///
    /// An additional hash is included on the balance with a name matching this value.
    #[serde(rename = "type")]
    pub type_: BankConnectionsResourceBalanceType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum BankConnectionsResourceBalanceType {
    Cash,
    Credit,
}

impl BankConnectionsResourceBalanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Cash => "cash",
            Self::Credit => "credit",
        }
    }
}

impl AsRef<str> for BankConnectionsResourceBalanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for BankConnectionsResourceBalanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for BankConnectionsResourceBalanceType {
    fn default() -> Self {
        Self::Cash
    }
}
