/// `Source` objects allow you to accept a variety of payment methods.
///
/// They represent a customer's payment instrument, and can be used with the Stripe API just like a `Card` object: once chargeable, they can be charged, or can be attached to customers.  Stripe doesn't recommend using the deprecated [Sources API](https://stripe.com/docs/api/sources). We recommend that you adopt the [PaymentMethods API](https://stripe.com/docs/api/payment_methods). This newer API provides access to our latest features and payment method types.  Related guides: [Sources API](https://stripe.com/docs/sources) and [Sources & Customers](https://stripe.com/docs/sources/customers).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Source {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<crate::generated::SourceTypeAchCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<crate::generated::SourceTypeAchDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::SourceTypeAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::SourceTypeAlipay>,

    /// A positive integer in the smallest currency unit (that is, 100 cents for $1.00, or 1 for ¥1, Japanese Yen being a zero-decimal currency) representing the total amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::SourceTypeAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::SourceTypeBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::SourceTypeCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::generated::SourceTypeCardPresent>,

    /// The client secret of the source.
    ///
    /// Used for client-side retrieval using a publishable key.
    pub client_secret: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub code_verification: Option<crate::generated::SourceCodeVerificationFlow>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    pub currency: Option<crate::currency::Currency>,

    /// The ID of the customer to which this source is attached.
    ///
    /// This will not be present when the source has not been attached to a customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::generated::SourceTypeEps>,

    /// The authentication `flow` of the source.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    pub flow: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::generated::SourceTypeGiropay>,

    /// Unique identifier for the object.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::SourceTypeIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::SourceTypeKlarna>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<crate::generated::SourceTypeMultibanco>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    pub owner: Option<crate::generated::SourceOwner>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::SourceTypeP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<crate::generated::SourceReceiverFlow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<crate::generated::SourceRedirectFlow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<crate::generated::SourceTypeSepaCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::SourceTypeSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::SourceTypeSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<crate::generated::SourceOrder>,

    /// Extra information about a source.
    ///
    /// This will appear on your customer's statement every time you charge the source.
    pub statement_descriptor: Option<String>,

    /// The status of the source, one of `canceled`, `chargeable`, `consumed`, `failed`, or `pending`.
    ///
    /// Only `chargeable` sources can be used to create a charge.
    pub status: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub three_d_secure: Option<crate::generated::SourceTypeThreeDSecure>,

    /// The `type` of the source.
    ///
    /// The `type` is a payment method, one of `ach_credit_transfer`, `ach_debit`, `alipay`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `multibanco`, `klarna`, `p24`, `sepa_debit`, `sofort`, `three_d_secure`, or `wechat`.
    /// An additional hash is included on the source with a name matching this value.
    /// It contains additional information specific to the [payment method](https://stripe.com/docs/sources) used.
    #[serde(rename = "type")]
    pub type_: SourceType,

    /// Either `reusable` or `single_use`.
    ///
    /// Whether this source should be reusable or not.
    /// Some source types may or may not be reusable by construction, while others may leave the option at creation.
    /// If an incompatible value is passed, an error will be returned.
    pub usage: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<crate::generated::SourceTypeWechat>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedDeleteCustomersCustomerSourcesId {
    PaymentSource(crate::generated::PaymentSource),
    DeletedPaymentSource(crate::generated::DeletedPaymentSource),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSourcesSourceParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_secret: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParams {
    /// Amount associated with the source.
    ///
    /// This is the amount for which the source will be chargeable once ready.
    /// Required for `single_use` sources.
    /// Not supported for `receiver` type sources, where charge amount may not be specified until funds land.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Three-letter [ISO code for the currency](https://stripe.com/docs/currencies) associated with the source.
    ///
    /// This is the currency for which the source will be chargeable once ready.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The `Customer` to whom the original source is attached to.
    ///
    /// Must be set when the original source is not a `Source` (e.g., `Card`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The authentication `flow` of the source to create.
    ///
    /// `flow` is one of `redirect`, `receiver`, `code_verification`, `none`.
    /// It is generally inferred unless a type supports multiple flows.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub flow: Option<PostSourcesParamsFlow>,

    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<PostSourcesParamsMandate>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// The source to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub original_source: Option<String>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<PostSourcesParamsOwner>,

    /// Optional parameters for the receiver flow.
    ///
    /// Can be set only if the source is a receiver (`flow` is `receiver`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub receiver: Option<PostSourcesParamsReceiver>,

    /// Parameters required for the redirect flow.
    ///
    /// Required if the source is authenticated by a redirect (`flow` is `redirect`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub redirect: Option<PostSourcesParamsRedirect>,

    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<PostSourcesParamsSourceOrder>,

    /// An arbitrary string to be displayed on your customer's statement.
    ///
    /// As an example, if your website is `RunClub` and the item you're charging for is a race ticket, you may want to specify a `statement_descriptor` of `RunClub 5K race ticket.` While many payment types will display this information, some may not display it at all.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub statement_descriptor: Option<String>,

    /// An optional token used to create the source.
    ///
    /// When passed, token properties will override source parameters.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub token: Option<String>,

    /// The `type` of the source to create.
    ///
    /// Required unless `customer` and `original_source` are specified (see the [Cloning card Sources](https://stripe.com/docs/sources/connect#cloning-card-sources) guide).
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<PostSourcesParamsUsage>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParams {
    /// Amount associated with the source.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mandate: Option<PostSourcesSourceParamsMandate>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// Information about the owner of the payment instrument that may be used or required by particular source types.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<PostSourcesSourceParamsOwner>,

    /// Information about the items and shipping associated with the source.
    ///
    /// Required for transactional credit (for example Klarna) sources before you can charge it.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub source_order: Option<PostSourcesSourceParamsSourceOrder>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceVerifyParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// The values needed to verify the source.
    pub values: Vec<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetSourcesSourceSourceTransactionsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum SourceType {
    AchCreditTransfer,
    AchDebit,
    AcssDebit,
    Alipay,
    AuBecsDebit,
    Bancontact,
    Card,
    CardPresent,
    Eps,
    Giropay,
    Ideal,
    Klarna,
    Multibanco,
    P24,
    SepaCreditTransfer,
    SepaDebit,
    Sofort,
    ThreeDSecure,
    Wechat,
}

impl SourceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AchCreditTransfer => "ach_credit_transfer",
            Self::AchDebit => "ach_debit",
            Self::AcssDebit => "acss_debit",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::Eps => "eps",
            Self::Giropay => "giropay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Multibanco => "multibanco",
            Self::P24 => "p24",
            Self::SepaCreditTransfer => "sepa_credit_transfer",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::ThreeDSecure => "three_d_secure",
            Self::Wechat => "wechat",
        }
    }
}

impl AsRef<str> for SourceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for SourceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for SourceType {
    fn default() -> Self {
        Self::AchCreditTransfer
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsFlow {
    CodeVerification,
    None,
    Receiver,
    Redirect,
}

impl PostSourcesParamsFlow {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::CodeVerification => "code_verification",
            Self::None => "none",
            Self::Receiver => "receiver",
            Self::Redirect => "redirect",
        }
    }
}

impl AsRef<str> for PostSourcesParamsFlow {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsFlow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsFlow {
    fn default() -> Self {
        Self::CodeVerification
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsMandate {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<PostSourcesParamsMandateAcceptance>,

    /// The amount specified by the mandate.
    ///
    /// (Leave null for a mandate covering all amounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The currency specified by the mandate.
    ///
    /// (Must match `currency` of the source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The interval of debits permitted by the mandate.
    ///
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PostSourcesParamsMandateInterval>,

    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    ///
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<PostSourcesParamsMandateNotificationMethod>,
}

/// Information about the owner of the payment instrument that may be used or required by particular source types.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsOwner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostSourcesParamsOwnerAddress>,

    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Optional parameters for the receiver flow.
///
/// Can be set only if the source is a receiver (`flow` is `receiver`).
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsReceiver {
    /// The method Stripe should use to request information needed to process a refund or mispayment.
    ///
    /// Either `email` (an email is sent directly to the customer) or `manual` (a `source.refund_attributes_required` event is sent to your webhooks endpoint).
    /// Refer to each payment method's documentation to learn which refund attributes may be required.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refund_attributes_method: Option<PostSourcesParamsReceiverRefundAttributesMethod>,
}

/// Parameters required for the redirect flow.
///
/// Required if the source is authenticated by a redirect (`flow` is `redirect`).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsRedirect {
    /// The URL you provide to redirect the customer back to you after they authenticated their payment.
    ///
    /// It can use your application URI scheme in the context of a mobile application.
    pub return_url: String,
}

/// Information about the items and shipping associated with the source.
///
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsSourceOrder {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostSourcesParamsSourceOrderItems>>,

    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostSourcesParamsSourceOrderShipping>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsUsage {
    Reusable,
    SingleUse,
}

impl PostSourcesParamsUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Reusable => "reusable",
            Self::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for PostSourcesParamsUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsUsage {
    fn default() -> Self {
        Self::Reusable
    }
}
/// Information about a mandate possibility attached to a source object (generally for bank debits) as well as its acceptance status.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsMandate {
    /// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acceptance: Option<PostSourcesSourceParamsMandateAcceptance>,

    /// The amount specified by the mandate.
    ///
    /// (Leave null for a mandate covering all amounts).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    /// The currency specified by the mandate.
    ///
    /// (Must match `currency` of the source).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    /// The interval of debits permitted by the mandate.
    ///
    /// Either `one_time` (just permitting a single debit), `scheduled` (with debits on an agreed schedule or for clearly-defined events), or `variable`(for debits with any frequency).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interval: Option<PostSourcesSourceParamsMandateInterval>,

    /// The method Stripe should use to notify the customer of upcoming debit instructions and/or mandate confirmation as required by the underlying debit network.
    ///
    /// Either `email` (an email is sent directly to the customer), `manual` (a `source.mandate_notification` event is sent to your webhooks endpoint and you should handle the notification) or `none` (the underlying debit network does not require any notification).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub notification_method: Option<PostSourcesSourceParamsMandateNotificationMethod>,
}

/// Information about the owner of the payment instrument that may be used or required by particular source types.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsOwner {
    /// Owner's address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostSourcesSourceParamsOwnerAddress>,

    /// Owner's email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Owner's full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Owner's phone number.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// Information about the items and shipping associated with the source.
///
/// Required for transactional credit (for example Klarna) sources before you can charge it.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsSourceOrder {
    /// List of items constituting the order.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub items: Option<Vec<PostSourcesSourceParamsSourceOrderItems>>,

    /// Shipping address for the order.
    ///
    /// Required if any of the SKUs are for products that have `shippable` set to true.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub shipping: Option<PostSourcesSourceParamsSourceOrderShipping>,
}

/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsMandateAcceptance {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The parameters required to store a mandate accepted offline.
    ///
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<PostSourcesParamsMandateAcceptanceOffline>,

    /// The parameters required to store a mandate accepted online.
    ///
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<PostSourcesParamsMandateAcceptanceOnline>,

    /// The status of the mandate acceptance.
    ///
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: PostSourcesParamsMandateAcceptanceStatus,

    /// The type of acceptance information included with the mandate.
    ///
    /// Either `online` or `offline`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostSourcesParamsMandateAcceptanceType>,

    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}

impl PostSourcesParamsMandateInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::Scheduled => "scheduled",
            Self::Variable => "variable",
        }
    }
}

impl AsRef<str> for PostSourcesParamsMandateInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsMandateInterval {
    fn default() -> Self {
        Self::OneTime
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}

impl PostSourcesParamsMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeprecatedNone => "deprecated_none",
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
            Self::StripeEmail => "stripe_email",
        }
    }
}

impl AsRef<str> for PostSourcesParamsMandateNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsMandateNotificationMethod {
    fn default() -> Self {
        Self::DeprecatedNone
    }
}
/// Owner's address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsOwnerAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsReceiverRefundAttributesMethod {
    Email,
    Manual,
    None,
}

impl PostSourcesParamsReceiverRefundAttributesMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PostSourcesParamsReceiverRefundAttributesMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsReceiverRefundAttributesMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsReceiverRefundAttributesMethod {
    fn default() -> Self {
        Self::Email
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsSourceOrderItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// The quantity of this order item.
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostSourcesParamsSourceOrderItemsType>,
}

/// Shipping address for the order.
///
/// Required if any of the SKUs are for products that have `shippable` set to true.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsSourceOrderShipping {
    /// Shipping address.
    pub address: PostSourcesParamsSourceOrderShippingAddress,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// The parameters required to notify Stripe of a mandate acceptance or refusal by the customer.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsMandateAcceptance {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The parameters required to store a mandate accepted offline.
    ///
    /// Should only be set if `mandate[type]` is `offline`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offline: Option<PostSourcesSourceParamsMandateAcceptanceOffline>,

    /// The parameters required to store a mandate accepted online.
    ///
    /// Should only be set if `mandate[type]` is `online`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub online: Option<PostSourcesSourceParamsMandateAcceptanceOnline>,

    /// The status of the mandate acceptance.
    ///
    /// Either `accepted` (the mandate was accepted) or `refused` (the mandate was refused).
    pub status: PostSourcesSourceParamsMandateAcceptanceStatus,

    /// The type of acceptance information included with the mandate.
    ///
    /// Either `online` or `offline`.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostSourcesSourceParamsMandateAcceptanceType>,

    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesSourceParamsMandateInterval {
    OneTime,
    Scheduled,
    Variable,
}

impl PostSourcesSourceParamsMandateInterval {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::OneTime => "one_time",
            Self::Scheduled => "scheduled",
            Self::Variable => "variable",
        }
    }
}

impl AsRef<str> for PostSourcesSourceParamsMandateInterval {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesSourceParamsMandateInterval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesSourceParamsMandateInterval {
    fn default() -> Self {
        Self::OneTime
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesSourceParamsMandateNotificationMethod {
    DeprecatedNone,
    Email,
    Manual,
    None,
    StripeEmail,
}

impl PostSourcesSourceParamsMandateNotificationMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::DeprecatedNone => "deprecated_none",
            Self::Email => "email",
            Self::Manual => "manual",
            Self::None => "none",
            Self::StripeEmail => "stripe_email",
        }
    }
}

impl AsRef<str> for PostSourcesSourceParamsMandateNotificationMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesSourceParamsMandateNotificationMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesSourceParamsMandateNotificationMethod {
    fn default() -> Self {
        Self::DeprecatedNone
    }
}
/// Owner's address.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsOwnerAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line1: Option<String>,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsSourceOrderItems {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub amount: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub currency: Option<crate::currency::Currency>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,

    /// The ID of the SKU being ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parent: Option<String>,

    /// The quantity of this order item.
    ///
    /// When type is `sku`, this is the number of instances of the SKU to be ordered.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<u64>,

    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostSourcesSourceParamsSourceOrderItemsType>,
}

/// Shipping address for the order.
///
/// Required if any of the SKUs are for products that have `shippable` set to true.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsSourceOrderShipping {
    /// Shipping address.
    pub address: PostSourcesSourceParamsSourceOrderShippingAddress,

    /// The delivery service that shipped a physical product, such as Fedex, UPS, USPS, etc.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub carrier: Option<String>,

    /// Recipient name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Recipient phone (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,

    /// The tracking number for a physical product, obtained from the delivery service.
    ///
    /// If multiple tracking numbers were generated for this purchase, please separate them with commas.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tracking_number: Option<String>,
}

/// The parameters required to store a mandate accepted offline.
///
/// Should only be set if `mandate[type]` is `offline`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsMandateAcceptanceOffline {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: String,
}

/// The parameters required to store a mandate accepted online.
///
/// Should only be set if `mandate[type]` is `online`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsMandateAcceptanceOnline {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl PostSourcesParamsMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl AsRef<str> for PostSourcesParamsMandateAcceptanceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsMandateAcceptanceStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsMandateAcceptanceType {
    Offline,
    Online,
}

impl PostSourcesParamsMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for PostSourcesParamsMandateAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsMandateAcceptanceType {
    fn default() -> Self {
        Self::Offline
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesParamsSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl PostSourcesParamsSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
            Self::Shipping => "shipping",
            Self::Sku => "sku",
            Self::Tax => "tax",
        }
    }
}

impl AsRef<str> for PostSourcesParamsSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesParamsSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesParamsSourceOrderItemsType {
    fn default() -> Self {
        Self::Discount
    }
}
/// Shipping address.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesParamsSourceOrderShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: String,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

/// The parameters required to store a mandate accepted offline.
///
/// Should only be set if `mandate[type]` is `offline`.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsMandateAcceptanceOffline {
    /// An email to contact you with if a copy of the mandate is requested, required if `type` is `offline`.
    pub contact_email: String,
}

/// The parameters required to store a mandate accepted online.
///
/// Should only be set if `mandate[type]` is `online`.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsMandateAcceptanceOnline {
    /// The Unix timestamp (in seconds) when the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub date: Option<crate::params::Timestamp>,

    /// The IP address from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ip: Option<String>,

    /// The user agent of the browser from which the mandate was accepted or refused by the customer.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_agent: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesSourceParamsMandateAcceptanceStatus {
    Accepted,
    Pending,
    Refused,
    Revoked,
}

impl PostSourcesSourceParamsMandateAcceptanceStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Accepted => "accepted",
            Self::Pending => "pending",
            Self::Refused => "refused",
            Self::Revoked => "revoked",
        }
    }
}

impl AsRef<str> for PostSourcesSourceParamsMandateAcceptanceStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesSourceParamsMandateAcceptanceStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesSourceParamsMandateAcceptanceStatus {
    fn default() -> Self {
        Self::Accepted
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesSourceParamsMandateAcceptanceType {
    Offline,
    Online,
}

impl PostSourcesSourceParamsMandateAcceptanceType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Offline => "offline",
            Self::Online => "online",
        }
    }
}

impl AsRef<str> for PostSourcesSourceParamsMandateAcceptanceType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesSourceParamsMandateAcceptanceType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesSourceParamsMandateAcceptanceType {
    fn default() -> Self {
        Self::Offline
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostSourcesSourceParamsSourceOrderItemsType {
    Discount,
    Shipping,
    Sku,
    Tax,
}

impl PostSourcesSourceParamsSourceOrderItemsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Discount => "discount",
            Self::Shipping => "shipping",
            Self::Sku => "sku",
            Self::Tax => "tax",
        }
    }
}

impl AsRef<str> for PostSourcesSourceParamsSourceOrderItemsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostSourcesSourceParamsSourceOrderItemsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostSourcesSourceParamsSourceOrderItemsType {
    fn default() -> Self {
        Self::Discount
    }
}
/// Shipping address.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostSourcesSourceParamsSourceOrderShippingAddress {
    /// City, district, suburb, town, or village.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub city: Option<String>,

    /// Two-letter country code ([ISO 3166-1 alpha-2](https://en.wikipedia.org/wiki/ISO_3166-1_alpha-2)).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub country: Option<String>,

    /// Address line 1 (e.g., street, PO Box, or company name).
    pub line1: String,

    /// Address line 2 (e.g., apartment, suite, unit, or building).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub line2: Option<String>,

    /// ZIP or postal code.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub postal_code: Option<String>,

    /// State, county, province, or region.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state: Option<String>,
}

pub fn delete_customers_customer_sources_id(
    client: &crate::Client,
    customer: String,
    id: String,
) -> crate::Response<ReturnedDeleteCustomersCustomerSourcesId> {
    client.delete(&format!("/customers/{customer}/sources/{id}", customer = customer, id = id))
}

pub fn get_sources_source(
    client: &crate::Client,
    source: String,
    params: GetSourcesSourceParams,
) -> crate::Response<crate::generated::Source> {
    client.get_query(&format!("/sources/{source}", source = source), params)
}

pub fn post_sources(
    client: &crate::Client,
    params: PostSourcesParams,
) -> crate::Response<crate::generated::Source> {
    client.post_form("/sources", params)
}

pub fn post_sources_source(
    client: &crate::Client,
    source: String,
    params: PostSourcesSourceParams,
) -> crate::Response<crate::generated::Source> {
    client.post_form(&format!("/sources/{source}", source = source), params)
}

pub fn post_sources_source_verify(
    client: &crate::Client,
    source: String,
    params: PostSourcesSourceVerifyParams,
) -> crate::Response<crate::generated::Source> {
    client.post_form(&format!("/sources/{source}/verify", source = source), params)
}

pub fn get_sources_source_source_transactions(
    client: &crate::Client,
    source: String,
    params: GetSourcesSourceSourceTransactionsParams,
) -> crate::Response<crate::params::List<crate::generated::SourceTransaction>> {
    client.get_query(&format!("/sources/{source}/source_transactions", source = source), params)
}
