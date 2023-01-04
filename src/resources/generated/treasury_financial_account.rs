/// Stripe Treasury provides users with a container for money called a FinancialAccount that is separate from their Payments balance.
/// FinancialAccounts serve as the source and destination of Treasury’s money movement APIs.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TreasuryFinancialAccount {
    /// The array of paths to active Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub active_features: Option<Vec<TreasuryFinancialAccountActiveFeatures>>,

    pub balance: crate::generated::TreasuryFinancialAccountsResourceBalance,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    pub country: String,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<crate::generated::TreasuryFinancialAccountFeatures>,

    /// The set of credentials that resolve to a FinancialAccount.
    pub financial_addresses:
        Vec<crate::generated::TreasuryFinancialAccountsResourceFinancialAddress>,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    /// The array of paths to pending Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pending_features: Option<Vec<TreasuryFinancialAccountPendingFeatures>>,

    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions:
        Option<crate::generated::TreasuryFinancialAccountsResourcePlatformRestrictions>,

    /// The array of paths to restricted Features in the Features hash.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub restricted_features: Option<Vec<TreasuryFinancialAccountRestrictedFeatures>>,

    /// The enum specifying what state the account is in.
    pub status: TreasuryFinancialAccountStatus,

    pub status_details: crate::generated::TreasuryFinancialAccountsResourceStatusDetails,

    /// The currencies the FinancialAccount can hold a balance in.
    ///
    /// Three-letter [ISO currency code](https://www.iso.org/iso-4217-currency-codes.html), in lowercase.
    pub supported_currencies: Vec<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Encodes whether a FinancialAccount has access to a particular feature.
    ///
    /// Stripe or the platform can control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PostTreasuryFinancialAccountsParamsFeatures>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions: Option<PostTreasuryFinancialAccountsParamsPlatformRestrictions>,

    /// The currencies the FinancialAccount can hold a balance in.
    pub supported_currencies: Vec<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
    ///
    /// Stripe or the platform may control features via the requested field.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub features: Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeatures>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The set of functionalities that the platform can restrict on the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub platform_restrictions:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictions>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParams {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsCardIssuing>,

    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsDepositInsurance>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsFinancialAddresses>,

    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsInboundTransfers>,

    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsIntraStripeFlows>,

    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPayments>,

    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfers>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryFinancialAccountsParams {
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
pub struct GetTreasuryFinancialAccountsFinancialAccountParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTreasuryFinancialAccountsFinancialAccountFeaturesParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountActiveFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountActiveFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountActiveFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountActiveFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountActiveFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountPendingFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountPendingFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountPendingFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountPendingFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountPendingFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountRestrictedFeatures {
    CardIssuing,
    DepositInsurance,
    #[serde(rename = "financial_addresses.aba")]
    FinancialAddressesAba,
    #[serde(rename = "inbound_transfers.ach")]
    InboundTransfersAch,
    IntraStripeFlows,
    #[serde(rename = "outbound_payments.ach")]
    OutboundPaymentsAch,
    #[serde(rename = "outbound_payments.us_domestic_wire")]
    OutboundPaymentsUsDomesticWire,
    #[serde(rename = "outbound_transfers.ach")]
    OutboundTransfersAch,
    #[serde(rename = "outbound_transfers.us_domestic_wire")]
    OutboundTransfersUsDomesticWire,
    RemoteDepositCapture,
}

impl TreasuryFinancialAccountRestrictedFeatures {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CardIssuing => "card_issuing",
            Self::DepositInsurance => "deposit_insurance",
            Self::FinancialAddressesAba => "financial_addresses.aba",
            Self::InboundTransfersAch => "inbound_transfers.ach",
            Self::IntraStripeFlows => "intra_stripe_flows",
            Self::OutboundPaymentsAch => "outbound_payments.ach",
            Self::OutboundPaymentsUsDomesticWire => "outbound_payments.us_domestic_wire",
            Self::OutboundTransfersAch => "outbound_transfers.ach",
            Self::OutboundTransfersUsDomesticWire => "outbound_transfers.us_domestic_wire",
            Self::RemoteDepositCapture => "remote_deposit_capture",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountRestrictedFeatures {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountRestrictedFeatures {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountRestrictedFeatures {
    fn default() -> Self {
        Self::CardIssuing
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum TreasuryFinancialAccountStatus {
    Closed,
    Open,
}

impl TreasuryFinancialAccountStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Closed => "closed",
            Self::Open => "open",
        }
    }
}

impl AsRef<str> for TreasuryFinancialAccountStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for TreasuryFinancialAccountStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for TreasuryFinancialAccountStatus {
    fn default() -> Self {
        Self::Closed
    }
}
/// Encodes whether a FinancialAccount has access to a particular feature.
///
/// Stripe or the platform can control features via the requested field.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing: Option<PostTreasuryFinancialAccountsParamsFeaturesCardIssuing>,

    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance: Option<PostTreasuryFinancialAccountsParamsFeaturesDepositInsurance>,

    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses: Option<PostTreasuryFinancialAccountsParamsFeaturesFinancialAddresses>,

    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers: Option<PostTreasuryFinancialAccountsParamsFeaturesInboundTransfers>,

    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows: Option<PostTreasuryFinancialAccountsParamsFeaturesIntraStripeFlows>,

    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments: Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundPayments>,

    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers: Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfers>,
}

/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows: Option<PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows>,

    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows:
        Option<PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows>,
}

/// Encodes whether a FinancialAccount has access to a particular feature, with a status enum and associated `status_details`.
///
/// Stripe or the platform may control features via the requested field.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeatures {
    /// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_issuing:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesCardIssuing>,

    /// Represents whether this FinancialAccount is eligible for deposit insurance.
    ///
    /// Various factors determine the insurance amount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deposit_insurance:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesDepositInsurance>,

    /// Contains Features that add FinancialAddresses to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_addresses:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesFinancialAddresses>,

    /// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_transfers:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesInboundTransfers>,

    /// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub intra_stripe_flows:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesIntraStripeFlows>,

    /// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_payments:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPayments>,

    /// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_transfers:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfers>,
}

/// The set of functionalities that the platform can restrict on the FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictions {
    /// Restricts all inbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub inbound_flows:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows>,

    /// Restricts all outbound money movement.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub outbound_flows: Option<
        PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows,
    >,
}

/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsFinancialAddressesAba>,
}

/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsInboundTransfersAch>,
}

/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPaymentsAch>,

    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<
        PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPaymentsUsDomesticWire,
    >,
}

/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach:
        Option<PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfersAch>,

    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<
        PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfersUsDomesticWire,
    >,
}

/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba: Option<PostTreasuryFinancialAccountsParamsFeaturesFinancialAddressesAba>,
}

/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsParamsFeaturesInboundTransfersAch>,
}

/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundPaymentsAch>,

    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundPaymentsUsDomesticWire>,
}

/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfersAch>,

    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire:
        Option<PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfersUsDomesticWire>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTreasuryFinancialAccountsParamsPlatformRestrictionsInboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str> for PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostTreasuryFinancialAccountsParamsPlatformRestrictionsOutboundFlows {
    fn default() -> Self {
        Self::Restricted
    }
}
/// Encodes the FinancialAccount's ability to be used with the Issuing product, including attaching cards to and drawing funds from the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesCardIssuing {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Represents whether this FinancialAccount is eligible for deposit insurance.
///
/// Various factors determine the insurance amount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesDepositInsurance {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Contains Features that add FinancialAddresses to the FinancialAccount.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesFinancialAddresses {
    /// Adds an ABA FinancialAddress to the FinancialAccount.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aba:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesFinancialAddressesAba>,
}

/// Contains settings related to adding funds to a FinancialAccount from another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesInboundTransfers {
    /// Enables ACH Debits via the InboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesInboundTransfersAch>,
}

/// Represents the ability for the FinancialAccount to send money to, or receive money from other FinancialAccounts (for example, via OutboundPayment).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesIntraStripeFlows {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Includes Features related to initiating money movement out of the FinancialAccount to someone else's bucket of money.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPayments {
    /// Enables ACH transfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach: Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPaymentsAch>,

    /// Enables US domestic wire tranfers via the OutboundPayments API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<
        PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire,
    >,
}

/// Contains a Feature and settings related to moving money out of the FinancialAccount into another Account with the same owner.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfers {
    /// Enables ACH transfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach:
        Option<PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfersAch>,

    /// Enables US domestic wire tranfers via the OutboundTransfers API.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_domestic_wire: Option<
        PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire,
    >,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows {
    Restricted,
    Unrestricted,
}

impl PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str>
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsInboundFlows
{
    fn default() -> Self {
        Self::Restricted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows {
    Restricted,
    Unrestricted,
}

impl PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Restricted => "restricted",
            Self::Unrestricted => "unrestricted",
        }
    }
}

impl AsRef<str>
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows
{
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default
    for PostTreasuryFinancialAccountsFinancialAccountParamsPlatformRestrictionsOutboundFlows
{
    fn default() -> Self {
        Self::Restricted
    }
}
/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH Debits via the InboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundPaymentsUsDomesticWire
{
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountFeaturesParamsOutboundTransfersUsDomesticWire
{
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH Debits via the InboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundPaymentsUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsParamsFeaturesOutboundTransfersUsDomesticWire {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Adds an ABA FinancialAddress to the FinancialAccount.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesFinancialAddressesAba {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH Debits via the InboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesInboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPaymentsAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundPayments API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundPaymentsUsDomesticWire
{
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables ACH transfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfersAch {
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

/// Enables US domestic wire tranfers via the OutboundTransfers API.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostTreasuryFinancialAccountsFinancialAccountParamsFeaturesOutboundTransfersUsDomesticWire
{
    /// Whether the FinancialAccount should have the Feature.
    pub requested: bool,
}

pub fn post_treasury_financial_accounts(
    client: &crate::Client,
    params: PostTreasuryFinancialAccountsParams,
) -> crate::Response<crate::generated::TreasuryFinancialAccount> {
    client.post_form("/treasury/financial_accounts", params)
}

pub fn post_treasury_financial_accounts_financial_account(
    client: &crate::Client,
    financial_account: String,
    params: PostTreasuryFinancialAccountsFinancialAccountParams,
) -> crate::Response<crate::generated::TreasuryFinancialAccount> {
    client.post_form(
        &format!(
            "/treasury/financial_accounts/{financial_account}",
            financial_account = financial_account
        ),
        params,
    )
}

pub fn post_treasury_financial_accounts_financial_account_features(
    client: &crate::Client,
    financial_account: String,
    params: PostTreasuryFinancialAccountsFinancialAccountFeaturesParams,
) -> crate::Response<crate::generated::TreasuryFinancialAccountFeatures> {
    client.post_form(
        &format!(
            "/treasury/financial_accounts/{financial_account}/features",
            financial_account = financial_account
        ),
        params,
    )
}

pub fn get_treasury_financial_accounts(
    client: &crate::Client,
    params: GetTreasuryFinancialAccountsParams,
) -> crate::Response<crate::params::List<crate::generated::TreasuryFinancialAccount>> {
    client.get_query("/treasury/financial_accounts", params)
}

pub fn get_treasury_financial_accounts_financial_account(
    client: &crate::Client,
    financial_account: String,
    params: GetTreasuryFinancialAccountsFinancialAccountParams,
) -> crate::Response<crate::generated::TreasuryFinancialAccount> {
    client.get_query(
        &format!(
            "/treasury/financial_accounts/{financial_account}",
            financial_account = financial_account
        ),
        params,
    )
}

pub fn get_treasury_financial_accounts_financial_account_features(
    client: &crate::Client,
    financial_account: String,
    params: GetTreasuryFinancialAccountsFinancialAccountFeaturesParams,
) -> crate::Response<crate::generated::TreasuryFinancialAccountFeatures> {
    client.get_query(
        &format!(
            "/treasury/financial_accounts/{financial_account}/features",
            financial_account = financial_account
        ),
        params,
    )
}
