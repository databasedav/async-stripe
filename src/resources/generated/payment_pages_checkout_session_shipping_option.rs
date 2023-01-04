#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentPagesCheckoutSessionShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,

    /// The shipping rate.
    pub shipping_rate: Vec<crate::generated::ShippingRate>,
}
