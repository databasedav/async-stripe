/// Some payment methods have no required amount that a customer must send.
/// Customers can be instructed to send any amount, and it can be made up of
/// multiple transactions.
///
/// As such, sources can have multiple associated transactions.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct SourceTransaction {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<crate::generated::SourceTransactionAchCreditTransferData>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the amount your customer has pushed to the receiver.
    pub amount: i64,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf_credit_transfer: Option<crate::generated::SourceTransactionChfCreditTransferData>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp_credit_transfer: Option<crate::generated::SourceTransactionGbpCreditTransferData>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paper_check: Option<crate::generated::SourceTransactionPaperCheckData>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<crate::generated::SourceTransactionSepaCreditTransferData>,

    /// The ID of the source this transaction is attached to.
    pub source: String,

    /// The status of the transaction, one of `succeeded`, `pending`, or `failed`.
    pub status: String,

    /// The type of source this transaction is attached to.
    #[serde(rename = "type")]
    pub type_: SourceTransactionType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceTransactionType {
    AchCreditTransfer,
    AchDebit,
    Alipay,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

impl SourceTransactionType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::Alipay => "alipay",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::Eps => "eps",
            Self::Giropay => "giropay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Multibanco => "multibanco",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::ThreeDSecure => "three_d_secure",
            Self::Wechat => "wechat",
        }
    }
}

impl AsRef<str> for SourceTransactionType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceTransactionType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SourceTransactionType {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}
