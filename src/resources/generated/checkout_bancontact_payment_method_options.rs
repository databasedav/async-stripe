#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct CheckoutBancontactPaymentMethodOptions {
    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<CheckoutBancontactPaymentMethodOptionsSetupFutureUsage>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    None,
}

impl CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CheckoutBancontactPaymentMethodOptionsSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}