#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourcePaymentSettings {
    /// The amount of the application fee (if any) that will be requested to be applied to the payment and transferred to the application owner's Stripe account.
    pub application_fee_amount: Option<i64>,

    /// Indicates whether order has been opted into using [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods) to manage payment method types.
    pub automatic_payment_methods:
        Option<crate::generated::OrdersV2ResourceAutomaticPaymentMethods>,

    /// PaymentMethod-specific configuration to provide to the order's PaymentIntent.
    pub payment_method_options: Option<crate::generated::OrdersV2ResourcePaymentMethodOptions>,

    /// The list of [payment method types](https://stripe.com/docs/payments/payment-methods/overview) to provide to the order's PaymentIntent.
    ///
    /// Do not include this attribute if you prefer to manage your payment methods from the [Stripe Dashboard](https://dashboard.stripe.com/settings/payment_methods).
    pub payment_method_types: Option<Vec<OrdersV2ResourcePaymentSettingsPaymentMethodTypes>>,

    /// The URL to redirect the customer to after they authenticate their payment.
    pub return_url: Option<String>,

    /// For non-card charges, you can use this value as the complete description that appears on your customers' statements.
    ///
    /// Must contain at least one letter, maximum 22 characters.
    pub statement_descriptor: Option<String>,

    /// Provides information about a card payment that customers see on their statements.
    ///
    /// Concatenated with the prefix (shortened descriptor) or statement descriptor that’s set on the account to form the complete statement descriptor.
    /// Maximum 22 characters for the concatenated descriptor.
    pub statement_descriptor_suffix: Option<String>,

    /// Provides configuration for completing a transfer for the order after it is paid.
    pub transfer_data: Option<crate::generated::OrdersV2ResourceTransferData>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    AcssDebit,
    AfterpayClearpay,
    Alipay,
    AuBecsDebit,
    BacsDebit,
    Bancontact,
    Card,
    CustomerBalance,
    Eps,
    Fpx,
    Giropay,
    Grabpay,
    Ideal,
    Klarna,
    Link,
    Oxxo,
    P24,
    SepaDebit,
    Sofort,
    WechatPay,
}

impl OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::AcssDebit => "acss_debit",
            Self::AfterpayClearpay => "afterpay_clearpay",
            Self::Alipay => "alipay",
            Self::AuBecsDebit => "au_becs_debit",
            Self::BacsDebit => "bacs_debit",
            Self::Bancontact => "bancontact",
            Self::Card => "card",
            Self::CustomerBalance => "customer_balance",
            Self::Eps => "eps",
            Self::Fpx => "fpx",
            Self::Giropay => "giropay",
            Self::Grabpay => "grabpay",
            Self::Ideal => "ideal",
            Self::Klarna => "klarna",
            Self::Link => "link",
            Self::Oxxo => "oxxo",
            Self::P24 => "p24",
            Self::SepaDebit => "sepa_debit",
            Self::Sofort => "sofort",
            Self::WechatPay => "wechat_pay",
        }
    }
}

impl AsRef<str> for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersV2ResourcePaymentSettingsPaymentMethodTypes {
    fn default() -> Self {
        Self::AcssDebit
    }
}
