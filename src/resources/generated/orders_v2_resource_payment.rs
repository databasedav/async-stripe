#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct OrdersV2ResourcePayment {
    /// ID of the payment intent associated with this order.
    ///
    /// Null when the order is `open`.
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,

    /// Settings describing how the order should configure generated PaymentIntents.
    pub settings: Option<crate::generated::OrdersV2ResourcePaymentSettings>,

    /// The status of the underlying payment associated with this order, if any.
    ///
    /// Null when the order is `open`.
    pub status: Option<OrdersV2ResourcePaymentStatus>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum OrdersV2ResourcePaymentStatus {
    Canceled,
    Complete,
    NotRequired,
    Processing,
    RequiresAction,
    RequiresCapture,
    RequiresConfirmation,
    RequiresPaymentMethod,
}

impl OrdersV2ResourcePaymentStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Complete => "complete",
            Self::NotRequired => "not_required",
            Self::Processing => "processing",
            Self::RequiresAction => "requires_action",
            Self::RequiresCapture => "requires_capture",
            Self::RequiresConfirmation => "requires_confirmation",
            Self::RequiresPaymentMethod => "requires_payment_method",
        }
    }
}

impl AsRef<str> for OrdersV2ResourcePaymentStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for OrdersV2ResourcePaymentStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for OrdersV2ResourcePaymentStatus {
    fn default() -> Self {
        Self::Canceled
    }
}
