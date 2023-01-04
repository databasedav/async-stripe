#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CheckoutSessionPaymentMethodOptions {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub acss_debit: Option<crate::generated::CheckoutAcssDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub affirm: Option<crate::generated::CheckoutAffirmPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub afterpay_clearpay: Option<crate::generated::CheckoutAfterpayClearpayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub alipay: Option<crate::generated::CheckoutAlipayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub au_becs_debit: Option<crate::generated::CheckoutAuBecsDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bacs_debit: Option<crate::generated::CheckoutBacsDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub bancontact: Option<crate::generated::CheckoutBancontactPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub boleto: Option<crate::generated::CheckoutBoletoPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub card: Option<crate::generated::CheckoutCardPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub customer_balance: Option<crate::generated::CheckoutCustomerBalancePaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eps: Option<crate::generated::CheckoutEpsPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub fpx: Option<crate::generated::CheckoutFpxPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub giropay: Option<crate::generated::CheckoutGiropayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub grabpay: Option<crate::generated::CheckoutGrabPayPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ideal: Option<crate::generated::CheckoutIdealPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub klarna: Option<crate::generated::CheckoutKlarnaPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub konbini: Option<crate::generated::CheckoutKonbiniPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub oxxo: Option<crate::generated::CheckoutOxxoPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub p24: Option<crate::generated::CheckoutP24PaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub paynow: Option<crate::generated::CheckoutPaynowPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub pix: Option<crate::generated::CheckoutPixPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sepa_debit: Option<crate::generated::CheckoutSepaDebitPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sofort: Option<crate::generated::CheckoutSofortPaymentMethodOptions>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub us_bank_account: Option<crate::generated::CheckoutUsBankAccountPaymentMethodOptions>,
}
