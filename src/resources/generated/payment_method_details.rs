#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_credit_transfer: Option<crate::generated::PaymentMethodDetailsAchCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ach_debit: Option<crate::generated::PaymentMethodDetailsAchDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::PaymentMethodDetailsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<crate::generated::PaymentMethodDetailsAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<crate::generated::PaymentMethodDetailsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::PaymentFlowsPrivatePaymentMethodsAlipayDetails>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::PaymentMethodDetailsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::PaymentMethodDetailsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::PaymentMethodDetailsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::PaymentMethodDetailsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::generated::PaymentMethodDetailsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::PaymentMethodDetailsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::generated::PaymentMethodDetailsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<crate::generated::PaymentMethodDetailsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::generated::PaymentMethodDetailsEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<crate::generated::PaymentMethodDetailsFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::generated::PaymentMethodDetailsGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<crate::generated::PaymentMethodDetailsGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::PaymentMethodDetailsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<crate::generated::PaymentMethodDetailsInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::PaymentMethodDetailsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<crate::generated::PaymentMethodDetailsKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::PaymentMethodDetailsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multibanco: Option<crate::generated::PaymentMethodDetailsMultibanco>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::generated::PaymentMethodDetailsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::PaymentMethodDetailsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<crate::generated::PaymentMethodDetailsPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<crate::generated::PaymentMethodDetailsPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<crate::generated::PaymentMethodDetailsPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_credit_transfer: Option<crate::generated::PaymentMethodDetailsSepaCreditTransfer>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::PaymentMethodDetailsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::PaymentMethodDetailsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub stripe_account: Option<crate::generated::PaymentMethodDetailsStripeAccount>,

    /// The type of transaction-specific details of the payment method used in the payment, one of `ach_credit_transfer`, `ach_debit`, `acss_debit`, `alipay`, `au_becs_debit`, `bancontact`, `card`, `card_present`, `eps`, `giropay`, `ideal`, `klarna`, `multibanco`, `p24`, `sepa_debit`, `sofort`, `stripe_account`, or `wechat`.
    /// An additional hash is included on `payment_method_details` with a name matching this value.
    /// It contains information specific to the payment method.
    #[serde(rename = "type")]
    pub type_: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::PaymentMethodDetailsUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat: Option<crate::generated::PaymentMethodDetailsWechat>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<crate::generated::PaymentMethodDetailsWechatPay>,
}
