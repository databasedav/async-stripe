#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentMethodOptionsKonbini {
    /// An optional 10 to 11 digit numeric-only string determining the confirmation code at applicable convenience stores.
    pub confirmation_number: Option<String>,

    /// The number of calendar days (between 1 and 60) after which Konbini payment instructions will expire.
    ///
    /// For example, if a PaymentIntent is confirmed with Konbini and `expires_after_days` set to 2 on Monday JST, the instructions will expire on Wednesday 23:59:59 JST.
    pub expires_after_days: Option<u32>,

    /// The timestamp at which the Konbini payment instructions will expire.
    ///
    /// Only one of `expires_after_days` or `expires_at` may be set.
    pub expires_at: Option<crate::resources::Scheduled>,

    /// A product descriptor of up to 22 characters, which will appear to customers at the convenience store.
    pub product_description: Option<String>,

    /// Indicates that you intend to make future payments with this PaymentIntent's payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the PaymentIntent's Customer, if present, after the PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<PaymentMethodOptionsKonbiniSetupFutureUsage>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum PaymentMethodOptionsKonbiniSetupFutureUsage {
    None,
}

impl PaymentMethodOptionsKonbiniSetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for PaymentMethodOptionsKonbiniSetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}