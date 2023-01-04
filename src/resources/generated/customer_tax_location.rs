#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct CustomerTaxLocation {
    /// The customer's country as identified by Stripe Tax.
    pub country: String,

    /// The data source used to infer the customer's location.
    pub source: CustomerTaxLocationSource,

    /// The customer's state, county, province, or region as identified by Stripe Tax.
    pub state: Option<String>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum CustomerTaxLocationSource {
    BillingAddress,
    IpAddress,
    PaymentMethod,
    ShippingDestination,
}

impl CustomerTaxLocationSource {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::BillingAddress => "billing_address",
            Self::IpAddress => "ip_address",
            Self::PaymentMethod => "payment_method",
            Self::ShippingDestination => "shipping_destination",
        }
    }
}

impl AsRef<str> for CustomerTaxLocationSource {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for CustomerTaxLocationSource {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for CustomerTaxLocationSource {
    fn default() -> Self {
        Self::BillingAddress
    }
}
