#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct ShippingRateDeliveryEstimate {
    /// The upper bound of the estimated range.
    ///
    /// If empty, represents no upper bound i.e., infinite.
    pub maximum: Option<crate::generated::ShippingRateDeliveryEstimateBound>,

    /// The lower bound of the estimated range.
    ///
    /// If empty, represents no lower bound.
    pub minimum: Option<crate::generated::ShippingRateDeliveryEstimateBound>,
}
