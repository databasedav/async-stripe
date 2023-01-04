/// You can reverse some [ReceivedDebits](https://stripe.com/docs/api#received_debits) depending on their network and source flow.
///
/// Reversing a ReceivedDebit leads to the creation of a new object known as a DebitReversal.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryDebitReversal {
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
    pub financial_account: Option<String>,

    /// A [hosted transaction receipt](https://stripe.com/docs/treasury/moving-money/regulatory-receipts) URL that is provided when money movement is considered regulated under Stripe's money transmission licenses.
    pub hosted_regulatory_receipt_url: Option<String>,

    /// Unique identifier for the object.
    pub id: String,

    /// Other flows linked to a DebitReversal.
    pub linked_flows:
        Option<crate::generated::TreasuryReceivedDebitsResourceDebitReversalLinkedFlows>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: crate::params::Metadata,

    /// The rails used to reverse the funds.
    pub network: TreasuryDebitReversalNetwork,

    /// The ReceivedDebit being reversed.
    pub received_debit: String,

    /// Status of the DebitReversal.
    pub status: TreasuryDebitReversalStatus,

    pub status_transitions: crate::generated::TreasuryReceivedDebitsResourceStatusTransitions,

    /// The Transaction associated with this object.
    pub transaction: Option<Vec<crate::generated::TreasuryTransaction>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryDebitReversalsParams {
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

    /// The ReceivedDebit to reverse.
    pub received_debit: String,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryDebitReversalsDebitReversalParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryDebitReversalsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    pub financial_account: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub received_debit: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub resolution: Option<GetTreasuryDebitReversalsParamsResolution>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<GetTreasuryDebitReversalsParamsStatus>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalNetwork {
    Ach,
    Card,
}

impl TreasuryDebitReversalNetwork {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ach => "ach",
            Self::Card => "card",
        }
    }
}

impl AsRef<str> for TreasuryDebitReversalNetwork {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryDebitReversalNetwork {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryDebitReversalNetwork {
    fn default() -> Self {
        Self::Ach
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryDebitReversalStatus {
    Failed,
    Processing,
    Succeeded,
}

impl TreasuryDebitReversalStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Failed => "failed",
            Self::Processing => "processing",
            Self::Succeeded => "succeeded",
        }
    }
}

impl AsRef<str> for TreasuryDebitReversalStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryDebitReversalStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryDebitReversalStatus {
    fn default() -> Self {
        Self::Failed
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryDebitReversalsParamsResolution {
    Lost,
    Won,
}

impl GetTreasuryDebitReversalsParamsResolution {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Lost => "lost",
            Self::Won => "won",
        }
    }
}

impl AsRef<str> for GetTreasuryDebitReversalsParamsResolution {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryDebitReversalsParamsResolution {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryDebitReversalsParamsResolution {
    fn default() -> Self {
        Self::Lost
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetTreasuryDebitReversalsParamsStatus {
    Canceled,
    Completed,
    Processing,
}

impl GetTreasuryDebitReversalsParamsStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Completed => "completed",
            Self::Processing => "processing",
        }
    }
}

impl AsRef<str> for GetTreasuryDebitReversalsParamsStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetTreasuryDebitReversalsParamsStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetTreasuryDebitReversalsParamsStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
pub fn post_treasury_debit_reversals(
    client: &crate::Client,
    params: PostTreasuryDebitReversalsParams,
) -> crate::Response<crate::generated::TreasuryDebitReversal> {
    client.post_form("/treasury/debit_reversals", params)
}

pub fn get_treasury_debit_reversals_debit_reversal(
    client: &crate::Client,
    debit_reversal: String,
    params: GetTreasuryDebitReversalsDebitReversalParams,
) -> crate::Response<crate::generated::TreasuryDebitReversal> {
    client.get_query(
        &format!("/treasury/debit_reversals/{debit_reversal}", debit_reversal = debit_reversal),
        params,
    )
}

pub fn get_treasury_debit_reversals(
    client: &crate::Client,
    params: GetTreasuryDebitReversalsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryDebitReversal>> {
    client.get_query("/treasury/debit_reversals", params)
}
