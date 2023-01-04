#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct ShippingRateDeliveryEstimateBound {
    /// A unit of time.
    pub unit: ShippingRateDeliveryEstimateBoundUnit,

    /// Must be greater than 0.
    pub value: i64,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ShippingRateDeliveryEstimateBoundUnit {
    BusinessDay,
    Day,
    Hour,
    Month,
    Week,
}

impl ShippingRateDeliveryEstimateBoundUnit {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BusinessDay => "business_day",
            Self::Day => "day",
            Self::Hour => "hour",
            Self::Month => "month",
            Self::Week => "week",
        }
    }
}

impl AsRef<str> for ShippingRateDeliveryEstimateBoundUnit {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for ShippingRateDeliveryEstimateBoundUnit {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for ShippingRateDeliveryEstimateBoundUnit {
    fn default() -> Self {
        Self::BusinessDay
    }
}
