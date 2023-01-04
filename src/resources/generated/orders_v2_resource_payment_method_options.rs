#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourcePaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::PaymentIntentPaymentMethodOptionsAcssDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<crate::generated::OrdersPaymentMethodOptionsAfterpayClearpay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::PaymentMethodOptionsAlipay>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::PaymentMethodOptionsBancontact>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::OrdersV2ResourceCardPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<crate::generated::PaymentMethodOptionsCustomerBalance>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::PaymentMethodOptionsIdeal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::PaymentMethodOptionsKlarna>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub link: Option<crate::generated::PaymentIntentPaymentMethodOptionsLink>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::generated::PaymentMethodOptionsOxxo>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::PaymentMethodOptionsP24>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paypal: Option<crate::generated::PaymentMethodOptionsPaypal>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::PaymentIntentPaymentMethodOptionsSepaDebit>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::PaymentMethodOptionsSofort>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub wechat_pay: Option<crate::generated::PaymentMethodOptionsWechatPay>,
}
