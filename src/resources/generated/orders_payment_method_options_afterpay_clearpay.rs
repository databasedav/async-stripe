#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersPaymentMethodOptionsAfterpayClearpay {
    /// Controls when the funds will be captured from the customer's account.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub capture_method: Option<OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod>,

    /// Order identifier shown to the user in Afterpay's online portal.
    ///
    /// We recommend using a value that helps you answer any questions a customer might have about the payment.
    /// The identifier is limited to 128 characters and may contain only letters, digits, underscores, backslashes and dashes.
    pub reference: Option<String>,

    /// Indicates that you intend to make future payments with the payment method.
    ///
    /// Providing this parameter will [attach the payment method](https://stripe.com/docs/payments/save-during-payment) to the order's Customer, if present, after the order's PaymentIntent is confirmed and any required actions from the user are complete.
    ///
    /// If no Customer was provided, the payment method can still be [attached](https://stripe.com/docs/api/payment_methods/attach) to a Customer after the transaction completes.  When processing card payments, Stripe also uses `setup_future_usage` to dynamically optimize your payment flow and comply with regional legislation and network rules, such as [SCA](https://stripe.com/docs/strong-customer-authentication).  If `setup_future_usage` is already set and you are performing a request using a publishable key, you may only update the value from `on_session` to `off_session`.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub setup_future_usage: Option<OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    Automatic,
    Manual,
}

impl OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Automatic => "automatic",
            Self::Manual => "manual",
        }
    }
}

impl AsRef<str> for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersPaymentMethodOptionsAfterpayClearpayCaptureMethod {
    fn default() -> Self {
        Self::Automatic
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    None,
}

impl OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}

impl AsRef<str> for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersPaymentMethodOptionsAfterpayClearpaySetupFutureUsage {
    fn default() -> Self {
        Self::None
    }
}
