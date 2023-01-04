#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::PaymentIntentPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<crate::generated::PaymentMethodOptionsAffirm>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<crate::generated::PaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::PaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::PaymentIntentPaymentMethodOptionsAuBecsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::PaymentMethodOptionsBacsDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::PaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub blik: Option<crate::generated::PaymentIntentPaymentMethodOptionsBlik>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::generated::PaymentMethodOptionsBoleto>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::PaymentIntentPaymentMethodOptionsCard>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_present: Option<crate::generated::PaymentMethodOptionsCardPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<crate::generated::PaymentMethodOptionsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::generated::PaymentIntentPaymentMethodOptionsEps>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<crate::generated::PaymentMethodOptionsFpx>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::generated::PaymentMethodOptionsGiropay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<crate::generated::PaymentMethodOptionsGrabpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::PaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub interac_present: Option<crate::generated::PaymentMethodOptionsInteracPresent>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::PaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<crate::generated::PaymentMethodOptionsKonbini>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::PaymentIntentPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::generated::PaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::PaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<crate::generated::PaymentMethodOptionsPaynow>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<crate::generated::PaymentMethodOptionsPix>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub promptpay: Option<crate::generated::PaymentMethodOptionsPromptpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::PaymentIntentPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::PaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::PaymentIntentPaymentMethodOptionsUsBankAccount>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<crate::generated::PaymentMethodOptionsWechatPay>,
}
