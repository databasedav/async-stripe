#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct PaymentLinksResourceShippingOption {
    /// A non-negative integer in cents representing how much to charge.
    pub shipping_amount: i64,

    /// The ID of the Shipping Rate to use for this shipping option.
    pub shipping_rate: Vec<crate::generated::ShippingRate>,
}
