/// PaymentMethod objects represent your customer's payment instruments.
/// You can use them with [PaymentIntents](https://stripe.com/docs/payments/payment-intents) to collect payments or save them to
/// Customer objects to store instrument details for future payments.
///
/// Related guides: [Payment Methods](https://stripe.com/docs/payments/payment-methods) and [More Payment Scenarios](https://stripe.com/docs/payments/more-payment-scenarios).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethod {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::PaymentMethodAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<crate::generated::PaymentMethodAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<crate::generated::PaymentMethodAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::PaymentFlowsPrivatePaymentMethodsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::PaymentMethodAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::PaymentMethodBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::PaymentMethodBancontact>,

    pub billing_details: crate::generated::BillingDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::PaymentMethodBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::generated::PaymentMethodBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::PaymentMethodCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::generated::PaymentMethodCardPresent>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The ID of the Customer to which this PaymentMethod is saved.
    ///
    /// This will not be set when the PaymentMethod has not been saved to a Customer.
    pub customer: Option<Vec<crate::generated::Customer>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<crate::generated::PaymentMethodCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::generated::PaymentMethodEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<crate::generated::PaymentMethodFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::generated::PaymentMethodGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<crate::generated::PaymentMethodGrabpay>,

    /// Unique identifier for the object.
    pub id: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::PaymentMethodIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<crate::generated::PaymentMethodInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::PaymentMethodKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<crate::generated::PaymentMethodKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::PaymentMethodLink>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    pub metadata: Option<crate::params::Metadata>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::generated::PaymentMethodOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::PaymentMethodP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<crate::generated::PaymentMethodPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<crate::generated::PaymentMethodPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<crate::generated::PaymentMethodPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<crate::generated::RadarRadarOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::PaymentMethodSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::PaymentMethodSofort>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    pub type_: PaymentMethodType,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::PaymentMethodUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<crate::generated::PaymentMethodWechatPay>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParams {
    /// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostPaymentMethodsParamsAcssDebit>,

    /// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PostPaymentMethodsParamsAffirm>,

    /// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<PostPaymentMethodsParamsAfterpayClearpay>,

    /// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<PostPaymentMethodsParamsAlipay>,

    /// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PostPaymentMethodsParamsAuBecsDebit>,

    /// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PostPaymentMethodsParamsBacsDebit>,

    /// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<PostPaymentMethodsParamsBancontact>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PostPaymentMethodsParamsBillingDetails>,

    /// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PostPaymentMethodsParamsBlik>,

    /// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<PostPaymentMethodsParamsBoleto>,

    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    ///
    /// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
    /// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
    /// We strongly recommend using Stripe.js instead of interacting with this API directly.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostPaymentMethodsParamsCard>,

    /// The `Customer` to whom the original PaymentMethod is attached.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    /// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<PostPaymentMethodsParamsCustomerBalance>,

    /// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<PostPaymentMethodsParamsEps>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<PostPaymentMethodsParamsFpx>,

    /// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<PostPaymentMethodsParamsGiropay>,

    /// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<PostPaymentMethodsParamsGrabpay>,

    /// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<PostPaymentMethodsParamsIdeal>,

    /// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<PostPaymentMethodsParamsInteracPresent>,

    /// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<PostPaymentMethodsParamsKlarna>,

    /// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<PostPaymentMethodsParamsKonbini>,

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PostPaymentMethodsParamsLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<PostPaymentMethodsParamsOxxo>,

    /// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<PostPaymentMethodsParamsP24>,

    /// The PaymentMethod to share.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_method: Option<String>,

    /// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<PostPaymentMethodsParamsPaynow>,

    /// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<PostPaymentMethodsParamsPix>,

    /// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<PostPaymentMethodsParamsPromptpay>,

    /// Options to configure Radar.
    ///
    /// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub radar_options: Option<PostPaymentMethodsParamsRadarOptions>,

    /// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PostPaymentMethodsParamsSepaDebit>,

    /// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<PostPaymentMethodsParamsSofort>,

    /// The type of the PaymentMethod.
    ///
    /// An additional hash is included on the PaymentMethod with a name matching this value.
    /// It contains additional information specific to the PaymentMethod type.
    #[serde(rename = "type")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub type_: Option<PostPaymentMethodsParamsType>,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PostPaymentMethodsParamsUsBankAccount>,

    /// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<PostPaymentMethodsParamsWechatPay>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetPaymentMethodsPaymentMethodParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParams {
    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<PostPaymentMethodsPaymentMethodParamsAcssDebit>,

    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<PostPaymentMethodsPaymentMethodParamsAffirm>,

    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<PostPaymentMethodsPaymentMethodParamsAuBecsDebit>,

    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<PostPaymentMethodsPaymentMethodParamsBacsDebit>,

    /// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub billing_details: Option<PostPaymentMethodsPaymentMethodParamsBillingDetails>,

    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<PostPaymentMethodsPaymentMethodParamsBlik>,

    /// If this is a `card` PaymentMethod, this hash contains the user's card details.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<PostPaymentMethodsPaymentMethodParamsCard>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<PostPaymentMethodsPaymentMethodParamsLink>,

    /// Set of [key-value pairs](https://stripe.com/docs/api/metadata) that you can attach to an object.
    ///
    /// This can be useful for storing additional information about the object in a structured format.
    /// Individual keys can be unset by posting an empty value to them.
    /// All keys can be unset by posting an empty value to `metadata`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<crate::params::Metadata>,

    /// This is a legacy parameter that will be removed in the future.
    ///
    /// It is a hash that does not accept any keys.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<PostPaymentMethodsPaymentMethodParamsSepaDebit>,

    /// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<PostPaymentMethodsPaymentMethodParamsUsBankAccount>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct GetPaymentMethodsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,

    #[serde(rename = "type")]
    pub type_: GetPaymentMethodsParamsType,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodAttachParams {
    /// The ID of the customer to which to attach the PaymentMethod.
    pub customer: String,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodDetachParams {
    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    InteracPresent,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PaymentMethodType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::InteracPresent => "interac_present",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PaymentMethodType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodType {
    fn default() -> Self {
        Self::AcssDebit
    }
}
/// If this is an `acss_debit` PaymentMethod, this hash contains details about the ACSS Debit payment method.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsAcssDebit {
    /// Customer's bank account number.
    pub account_number: String,

    /// Institution number of the customer's bank.
    pub institution_number: String,

    /// Transit number of the customer's bank.
    pub transit_number: String,
}

/// If this is an `affirm` PaymentMethod, this hash contains details about the Affirm payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsAffirm {}

/// If this is an `AfterpayClearpay` PaymentMethod, this hash contains details about the AfterpayClearpay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsAfterpayClearpay {}

/// If this is an `Alipay` PaymentMethod, this hash contains details about the Alipay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsAlipay {}

/// If this is an `au_becs_debit` PaymentMethod, this hash contains details about the bank account.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsAuBecsDebit {
    /// The account number for the bank account.
    pub account_number: String,

    /// Bank-State-Branch number of the bank account.
    pub bsb_number: String,
}

/// If this is a `bacs_debit` PaymentMethod, this hash contains details about the Bacs Direct Debit bank account.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBacsDebit {
    /// Account number of the bank account that the funds will be debited from.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Sort code of the bank account.
    ///
    /// (e.g., `10-20-30`).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sort_code: Option<String>,
}

/// If this is a `bancontact` PaymentMethod, this hash contains details about the Bancontact payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBancontact {}

/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostPaymentMethodsParamsBillingDetailsAddress>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// If this is a `blik` PaymentMethod, this hash contains details about the BLIK payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBlik {}

/// If this is a `boleto` PaymentMethod, this hash contains details about the Boleto payment method.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBoleto {
    /// The tax ID of the customer (CPF for individual consumers or CNPJ for businesses consumers).
    pub tax_id: String,
}

/// If this is a `card` PaymentMethod, this hash contains the user's card details.
///
/// For backwards compatibility, you can alternatively provide a Stripe token (e.g., for Apple Pay, Amex Express Checkout, or legacy Checkout) into the card hash with format `card: {token: "tok_visa"}`.
/// When providing a card number, you must meet the requirements for [PCI compliance](https://stripe.com/docs/security#validating-pci-compliance).
/// We strongly recommend using Stripe.js instead of interacting with this API directly.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsCard {
    CardDetailsParams(PostPaymentMethodsParamsCardCardDetailsParams),
    TokenParams(PostPaymentMethodsParamsCardTokenParams),
}

/// If this is a `customer_balance` PaymentMethod, this hash contains details about the CustomerBalance payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsCustomerBalance {}

/// If this is an `eps` PaymentMethod, this hash contains details about the EPS payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsEps {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<PostPaymentMethodsParamsEpsBank>,
}

/// If this is an `fpx` PaymentMethod, this hash contains details about the FPX payment method.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsFpx {
    /// Account holder type for FPX transaction.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostPaymentMethodsParamsFpxAccountHolderType>,

    /// The customer's bank.
    pub bank: PostPaymentMethodsParamsFpxBank,
}

/// If this is a `giropay` PaymentMethod, this hash contains details about the Giropay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsGiropay {}

/// If this is a `grabpay` PaymentMethod, this hash contains details about the GrabPay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsGrabpay {}

/// If this is an `ideal` PaymentMethod, this hash contains details about the iDEAL payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsIdeal {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<PostPaymentMethodsParamsIdealBank>,
}

/// If this is an `interac_present` PaymentMethod, this hash contains details about the Interac Present payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsInteracPresent {}

/// If this is a `klarna` PaymentMethod, this hash contains details about the Klarna payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsKlarna {
    /// Customer's date of birth.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dob: Option<PostPaymentMethodsParamsKlarnaDob>,
}

/// If this is a `konbini` PaymentMethod, this hash contains details about the Konbini payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsKonbini {}

/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsLink {}

/// If this is an `oxxo` PaymentMethod, this hash contains details about the OXXO payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsOxxo {}

/// If this is a `p24` PaymentMethod, this hash contains details about the P24 payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsP24 {
    /// The customer's bank.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bank: Option<PostPaymentMethodsParamsP24Bank>,
}

/// If this is a `paynow` PaymentMethod, this hash contains details about the PayNow payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsPaynow {}

/// If this is a `pix` PaymentMethod, this hash contains details about the Pix payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsPix {}

/// If this is a `promptpay` PaymentMethod, this hash contains details about the PromptPay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsPromptpay {}

/// Options to configure Radar.
///
/// See [Radar Session](https://stripe.com/docs/radar/radar-session) for more information.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsRadarOptions {
    /// A [Radar Session](https://stripe.com/docs/radar/radar-session) is a snapshot of the browser metadata and device details that help Radar make more accurate predictions on your payments.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub session: Option<String>,
}

/// If this is a `sepa_debit` PaymentMethod, this hash contains details about the SEPA debit bank account.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsSepaDebit {
    /// IBAN of the bank account.
    pub iban: String,
}

/// If this is a `sofort` PaymentMethod, this hash contains details about the SOFORT payment method.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsSofort {
    /// Two-letter ISO code representing the country the bank account is located in.
    pub country: PostPaymentMethodsParamsSofortCountry,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl PostPaymentMethodsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsType {
    fn default() -> Self {
        Self::AcssDebit
    }
}
/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsUsBankAccount {
    /// Account holder type: individual or company.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type: Option<PostPaymentMethodsParamsUsBankAccountAccountHolderType>,

    /// Account number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_number: Option<String>,

    /// Account type: checkings or savings.
    ///
    /// Defaults to checking if omitted.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_type: Option<PostPaymentMethodsParamsUsBankAccountAccountType>,

    /// The ID of a Financial Connections Account to use as a payment method.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub financial_connections_account: Option<String>,

    /// Routing number of the bank account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub routing_number: Option<String>,
}

/// If this is an `wechat_pay` PaymentMethod, this hash contains details about the wechat_pay payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsWechatPay {}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsAcssDebit {}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsAffirm {}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsAuBecsDebit {}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsBacsDebit {}

/// Billing information associated with the PaymentMethod that may be used or required by particular types of payment methods.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsBillingDetails {
    /// Billing address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub address: Option<PostPaymentMethodsPaymentMethodParamsBillingDetailsAddress>,

    /// Email address.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub email: Option<String>,

    /// Full name.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    /// Billing phone number (including extension).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub phone: Option<String>,
}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsBlik {}

/// If this is a `card` PaymentMethod, this hash contains the user's card details.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsCard {
    /// Two-digit number representing the card's expiration month.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_month: Option<i64>,

    /// Four-digit number representing the card's expiration year.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exp_year: Option<i64>,
}

/// If this is an `Link` PaymentMethod, this hash contains details about the Link payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsLink {}

/// This is a legacy parameter that will be removed in the future.
///
/// It is a hash that does not accept any keys.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsSepaDebit {}

/// If this is an `us_bank_account` PaymentMethod, this hash contains details about the US bank account payment method.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsUsBankAccount {
    /// Bank account type.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub account_holder_type:
        Option<PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum GetPaymentMethodsParamsType {
    AcssDebit,
    Affirm,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Blik,
    Boleto,
    Card,
    CardPresent,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Konbini,
    Link,
    Oxxo,
    P24,
    Paynow,
    Pix,
    Promptpay,
    SepaDebit,
    Sofort,
    UsBankAccount,
    WechatPay,
}

impl GetPaymentMethodsParamsType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::Affirm => "affirm",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Blik => "blik",
            Self::Boleto => "boleto",
            Self::Card => "card",
            Self::CardPresent => "card_present",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Konbini => "konbini",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::Paynow => "paynow",
            Self::Pix => "pix",
            Self::Promptpay => "promptpay",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::UsBankAccount => "us_bank_account",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for GetPaymentMethodsParamsType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for GetPaymentMethodsParamsType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for GetPaymentMethodsParamsType {
    fn default() -> Self {
        Self::AcssDebit
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsBillingDetailsAddress {
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

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsCardCardDetailsParams {
    /// The card's CVC.
    ///
    /// It is highly recommended to always include this value.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cvc: Option<String>,

    /// Two-digit number representing the card's expiration month.
    pub exp_month: i64,

    /// Four-digit number representing the card's expiration year.
    pub exp_year: i64,

    /// The card number, as a string without any separators.
    pub number: String,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsCardTokenParams {
    pub token: String,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsEpsBank {
    ArzteUndApothekerBank,
    AustrianAnadiBankAg,
    BankAustria,
    BankhausCarlSpangler,
    BankhausSchelhammerUndSchatteraAg,
    BawagPskAg,
    BksBankAg,
    BrullKallmusBankAg,
    BtvVierLanderBank,
    CapitalBankGraweGruppeAg,
    DeutscheBankAg,
    Dolomitenbank,
    EasybankAg,
    ErsteBankUndSparkassen,
    HypoAlpeadriabankInternationalAg,
    HypoBankBurgenlandAktiengesellschaft,
    HypoNoeLbFurNiederosterreichUWien,
    HypoOberosterreichSalzburgSteiermark,
    HypoTirolBankAg,
    HypoVorarlbergBankAg,
    MarchfelderBank,
    OberbankAg,
    RaiffeisenBankengruppeOsterreich,
    SchoellerbankAg,
    SpardaBankWien,
    VolksbankGruppe,
    VolkskreditbankAg,
    VrBankBraunau,
}

impl PostPaymentMethodsParamsEpsBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ArzteUndApothekerBank => "arzte_und_apotheker_bank",
            Self::AustrianAnadiBankAg => "austrian_anadi_bank_ag",
            Self::BankAustria => "bank_austria",
            Self::BankhausCarlSpangler => "bankhaus_carl_spangler",
            Self::BankhausSchelhammerUndSchatteraAg => "bankhaus_schelhammer_und_schattera_ag",
            Self::BawagPskAg => "bawag_psk_ag",
            Self::BksBankAg => "bks_bank_ag",
            Self::BrullKallmusBankAg => "brull_kallmus_bank_ag",
            Self::BtvVierLanderBank => "btv_vier_lander_bank",
            Self::CapitalBankGraweGruppeAg => "capital_bank_grawe_gruppe_ag",
            Self::DeutscheBankAg => "deutsche_bank_ag",
            Self::Dolomitenbank => "dolomitenbank",
            Self::EasybankAg => "easybank_ag",
            Self::ErsteBankUndSparkassen => "erste_bank_und_sparkassen",
            Self::HypoAlpeadriabankInternationalAg => "hypo_alpeadriabank_international_ag",
            Self::HypoBankBurgenlandAktiengesellschaft => "hypo_bank_burgenland_aktiengesellschaft",
            Self::HypoNoeLbFurNiederosterreichUWien => "hypo_noe_lb_fur_niederosterreich_u_wien",
            Self::HypoOberosterreichSalzburgSteiermark => "hypo_oberosterreich_salzburg_steiermark",
            Self::HypoTirolBankAg => "hypo_tirol_bank_ag",
            Self::HypoVorarlbergBankAg => "hypo_vorarlberg_bank_ag",
            Self::MarchfelderBank => "marchfelder_bank",
            Self::OberbankAg => "oberbank_ag",
            Self::RaiffeisenBankengruppeOsterreich => "raiffeisen_bankengruppe_osterreich",
            Self::SchoellerbankAg => "schoellerbank_ag",
            Self::SpardaBankWien => "sparda_bank_wien",
            Self::VolksbankGruppe => "volksbank_gruppe",
            Self::VolkskreditbankAg => "volkskreditbank_ag",
            Self::VrBankBraunau => "vr_bank_braunau",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsEpsBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsEpsBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsEpsBank {
    fn default() -> Self {
        Self::ArzteUndApothekerBank
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsFpxAccountHolderType {
    Company,
    Individual,
}

impl PostPaymentMethodsParamsFpxAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsFpxAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsFpxAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsFpxAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsFpxBank {
    AffinBank,
    Agrobank,
    AllianceBank,
    Ambank,
    BankIslam,
    BankMuamalat,
    BankOfChina,
    BankRakyat,
    Bsn,
    Cimb,
    DeutscheBank,
    HongLeongBank,
    Hsbc,
    Kfh,
    Maybank2e,
    Maybank2u,
    Ocbc,
    PbEnterprise,
    PublicBank,
    Rhb,
    StandardChartered,
    Uob,
}

impl PostPaymentMethodsParamsFpxBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AffinBank => "affin_bank",
            Self::Agrobank => "agrobank",
            Self::AllianceBank => "alliance_bank",
            Self::Ambank => "ambank",
            Self::BankIslam => "bank_islam",
            Self::BankMuamalat => "bank_muamalat",
            Self::BankOfChina => "bank_of_china",
            Self::BankRakyat => "bank_rakyat",
            Self::Bsn => "bsn",
            Self::Cimb => "cimb",
            Self::DeutscheBank => "deutsche_bank",
            Self::HongLeongBank => "hong_leong_bank",
            Self::Hsbc => "hsbc",
            Self::Kfh => "kfh",
            Self::Maybank2e => "maybank2e",
            Self::Maybank2u => "maybank2u",
            Self::Ocbc => "ocbc",
            Self::PbEnterprise => "pb_enterprise",
            Self::PublicBank => "public_bank",
            Self::Rhb => "rhb",
            Self::StandardChartered => "standard_chartered",
            Self::Uob => "uob",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsFpxBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsFpxBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsFpxBank {
    fn default() -> Self {
        Self::AffinBank
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsIdealBank {
    AbnAmro,
    AsnBank,
    Bunq,
    Handelsbanken,
    Ing,
    Knab,
    Moneyou,
    Rabobank,
    Regiobank,
    Revolut,
    SnsBank,
    TriodosBank,
    VanLanschot,
}

impl PostPaymentMethodsParamsIdealBank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AbnAmro => "abn_amro",
            Self::AsnBank => "asn_bank",
            Self::Bunq => "bunq",
            Self::Handelsbanken => "handelsbanken",
            Self::Ing => "ing",
            Self::Knab => "knab",
            Self::Moneyou => "moneyou",
            Self::Rabobank => "rabobank",
            Self::Regiobank => "regiobank",
            Self::Revolut => "revolut",
            Self::SnsBank => "sns_bank",
            Self::TriodosBank => "triodos_bank",
            Self::VanLanschot => "van_lanschot",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsIdealBank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsIdealBank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsIdealBank {
    fn default() -> Self {
        Self::AbnAmro
    }
}
/// Customer's date of birth.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsParamsKlarnaDob {
    /// The day of birth, between 1 and 31.
    pub day: i64,

    /// The month of birth, between 1 and 12.
    pub month: i64,

    /// The four-digit year of birth.
    pub year: i64,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsP24Bank {
    AliorBank,
    BankMillennium,
    BankNowyBfgSa,
    BankPekaoSa,
    BankiSpbdzielcze,
    Blik,
    BnpParibas,
    Boz,
    CitiHandlowy,
    CreditAgricole,
    Envelobank,
    EtransferPocztowy24,
    GetinBank,
    Ideabank,
    Ing,
    Inteligo,
    MbankMtransfer,
    NestPrzelew,
    NoblePay,
    PbacZIpko,
    PlusBank,
    SantanderPrzelew24,
    TmobileUsbugiBankowe,
    ToyotaBank,
    VolkswagenBank,
}

impl PostPaymentMethodsParamsP24Bank {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AliorBank => "alior_bank",
            Self::BankMillennium => "bank_millennium",
            Self::BankNowyBfgSa => "bank_nowy_bfg_sa",
            Self::BankPekaoSa => "bank_pekao_sa",
            Self::BankiSpbdzielcze => "banki_spbdzielcze",
            Self::Blik => "blik",
            Self::BnpParibas => "bnp_paribas",
            Self::Boz => "boz",
            Self::CitiHandlowy => "citi_handlowy",
            Self::CreditAgricole => "credit_agricole",
            Self::Envelobank => "envelobank",
            Self::EtransferPocztowy24 => "etransfer_pocztowy24",
            Self::GetinBank => "getin_bank",
            Self::Ideabank => "ideabank",
            Self::Ing => "ing",
            Self::Inteligo => "inteligo",
            Self::MbankMtransfer => "mbank_mtransfer",
            Self::NestPrzelew => "nest_przelew",
            Self::NoblePay => "noble_pay",
            Self::PbacZIpko => "pbac_z_ipko",
            Self::PlusBank => "plus_bank",
            Self::SantanderPrzelew24 => "santander_przelew24",
            Self::TmobileUsbugiBankowe => "tmobile_usbugi_bankowe",
            Self::ToyotaBank => "toyota_bank",
            Self::VolkswagenBank => "volkswagen_bank",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsP24Bank {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsP24Bank {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsP24Bank {
    fn default() -> Self {
        Self::AliorBank
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsSofortCountry {
    #[serde(rename = "AT")]
    At,
    #[serde(rename = "BE")]
    Be,
    #[serde(rename = "DE")]
    De,
    #[serde(rename = "ES")]
    Es,
    #[serde(rename = "IT")]
    It,
    #[serde(rename = "NL")]
    Nl,
}

impl PostPaymentMethodsParamsSofortCountry {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::At => "AT",
            Self::Be => "BE",
            Self::De => "DE",
            Self::Es => "ES",
            Self::It => "IT",
            Self::Nl => "NL",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsSofortCountry {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsSofortCountry {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsSofortCountry {
    fn default() -> Self {
        Self::At
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PostPaymentMethodsParamsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PostPaymentMethodsParamsUsBankAccountAccountType {
    Checking,
    Savings,
}

impl PostPaymentMethodsParamsUsBankAccountAccountType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Checking => "checking",
            Self::Savings => "savings",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsParamsUsBankAccountAccountType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsParamsUsBankAccountAccountType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsParamsUsBankAccountAccountType {
    fn default() -> Self {
        Self::Checking
    }
}
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostPaymentMethodsPaymentMethodParamsBillingDetailsAddress {
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
pub enum PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType {
    Company,
    Individual,
}

impl PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Company => "company",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PostPaymentMethodsPaymentMethodParamsUsBankAccountAccountHolderType {
    fn default() -> Self {
        Self::Company
    }
}
pub fn post_payment_methods(
    client: &crate::Client,
    params: PostPaymentMethodsParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.post_form("/payment_methods", params)
}

pub fn get_payment_methods_payment_method(
    client: &crate::Client,
    payment_method: String,
    params: GetPaymentMethodsPaymentMethodParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.get_query(
        &format!("/payment_methods/{payment_method}", payment_method = payment_method),
        params,
    )
}

pub fn post_payment_methods_payment_method(
    client: &crate::Client,
    payment_method: String,
    params: PostPaymentMethodsPaymentMethodParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.post_form(
        &format!("/payment_methods/{payment_method}", payment_method = payment_method),
        params,
    )
}

pub fn get_payment_methods(
    client: &crate::Client,
    params: GetPaymentMethodsParams,
) -> crate::Response<crate::params::List<crate::generated::PaymentMethod>> {
    client.get_query("/payment_methods", params)
}

pub fn post_payment_methods_payment_method_attach(
    client: &crate::Client,
    payment_method: String,
    params: PostPaymentMethodsPaymentMethodAttachParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.post_form(
        &format!("/payment_methods/{payment_method}/attach", payment_method = payment_method),
        params,
    )
}

pub fn post_payment_methods_payment_method_detach(
    client: &crate::Client,
    payment_method: String,
    params: PostPaymentMethodsPaymentMethodDetachParams,
) -> crate::Response<crate::generated::PaymentMethod> {
    client.post_form(
        &format!("/payment_methods/{payment_method}/detach", payment_method = payment_method),
        params,
    )
}
