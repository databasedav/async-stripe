/// You can reverse some [ReceivedCredits](https://stripe.com/docs/api#received_credits) depending on their network and source flow.
///
/// Reversing a ReceivedCredit leads to the creation of a new object known as a CreditReversal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryCreditReversal {
    /// Amount (in cents) transferred.
    pub amount: i64,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    ///
    /// Must be a [supported currency](https://stripe.com/docs/currencies).
    pub currency: crate::currency::Currency,

    /// The FinancialAccount to reverse funds from.
    pub financial_account: String,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The rails used to reverse the funds.
    pub network: TreasuryCreditReversalNetwork,

    /// The ReceivedCredit being reversed.
    pub received_credit: String,

    /// Status of the CreditReversal.
    pub status: TreasuryCreditReversalStatus,

    pub status_transitions: crate::generated::TreasuryReceivedCreditsResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Option<Vec<crate::generated::TreasuryTransaction>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryCreditReversalsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_credit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTreasuryCreditReversalsParamsStatus>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryCreditReversalsCreditReversalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryCreditReversalsParams {
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

    /// The ReceivedCredit to reverse.
    pub received_credit: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalNetwork {
    Ach,
    Stripe,
}

impl TreasuryCreditReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Stripe => "stripe",
        }
    }
}

impl AsRef<str> for TreasuryCreditReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryCreditReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryCreditReversalNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryCreditReversalStatus {
    Canceled,
    Posted,
    Processing,
}

impl TreasuryCreditReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Posted => "posted",
            Self::Processing => "processing",
        }
    }
}

impl AsRef<str> for TreasuryCreditReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryCreditReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryCreditReversalStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryCreditReversalsParamsStatus {
    Canceled,
    Posted,
    Processing,
}

impl GetTreasuryCreditReversalsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Posted => "posted",
            Self::Processing => "processing",
        }
    }
}

impl AsRef<str> for GetTreasuryCreditReversalsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryCreditReversalsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryCreditReversalsParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
pub fn get_treasury_credit_reversals(
    client: &crate::Client,
    params: GetTreasuryCreditReversalsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryCreditReversal>> {
    client.get_query("/treasury/credit_reversals", params)
}

pub fn get_treasury_credit_reversals_credit_reversal(
    client: &crate::Client,
    credit_reversal: String,
    params: GetTreasuryCreditReversalsCreditReversalParams,
) -> crate::Response<crate::generated::TreasuryCreditReversal> {
    client.get_query(
        &format!("/treasury/credit_reversals/{credit_reversal}", credit_reversal = credit_reversal),
        params,
    )
}

pub fn post_treasury_credit_reversals(
    client: &crate::Client,
    params: PostTreasuryCreditReversalsParams,
) -> crate::Response<crate::generated::TreasuryCreditReversal> {
    client.post_form("/treasury/credit_reversals", params)
}
