#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PaymentIntentNextActionCardAwaitNotification {
    /// The time that payment will be attempted.
    ///
    /// If customer approval is required, they need to provide approval before this time.
    pub charge_attempt_at: Option<crate::params::Timestamp>,

    /// For payments greater than INR 15000, the customer must provide explicit approval of the payment with their bank.
    ///
    /// For payments of lower amount, no customer action is required.
    pub customer_approval_required: Option<bool>,
}
