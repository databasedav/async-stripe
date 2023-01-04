#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct IssuingCardShipping {
    pub address: crate::generated::Address,

    /// The delivery company that shipped a card.
    pub carrier: Option<IssuingCardShippingCarrier>,

    /// Additional information that may be required for clearing customs.
    pub customs: Option<crate::generated::IssuingCardShippingCustoms>,

    /// A unix timestamp representing a best estimate of when the card will be delivered.
    pub eta: Option<crate::params::Timestamp>,

    /// Recipient name.
    pub name: String,

    /// The phone number of the receiver of the bulk shipment.
    ///
    /// This phone number will be provided to the shipping company, who might use it to contact the receiver in case of delivery issues.
    pub phone_number: Option<String>,

    /// Whether a signature is required for card delivery.
    ///
    /// This feature is only supported for US users.
    /// Standard shipping service does not support signature on delivery.
    /// The default value for standard shipping service is false and for express and priority services is true.
    pub require_signature: Option<bool>,

    /// Shipment service, such as `standard` or `express`.
    pub service: IssuingCardShippingService,

    /// The delivery status of the card.
    pub status: Option<IssuingCardShippingStatus>,

    /// A tracking number for a card shipment.
    pub tracking_number: Option<String>,

    /// A link to the shipping carrier's site where you can view detailed information about a card shipment.
    pub tracking_url: Option<String>,

    /// Packaging options.
    #[serde(rename = "type")]
    pub type_: IssuingCardShippingType,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingCarrier {
    Dhl,
    Fedex,
    RoyalMail,
    Usps,
}

impl IssuingCardShippingCarrier {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Dhl => "dhl",
            Self::Fedex => "fedex",
            Self::RoyalMail => "royal_mail",
            Self::Usps => "usps",
        }
    }
}

impl AsRef<str> for IssuingCardShippingCarrier {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingCarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardShippingCarrier {
    fn default() -> Self {
        Self::Dhl
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingService {
    Express,
    Priority,
    Standard,
}

impl IssuingCardShippingService {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Express => "express",
            Self::Priority => "priority",
            Self::Standard => "standard",
        }
    }
}

impl AsRef<str> for IssuingCardShippingService {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingService {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardShippingService {
    fn default() -> Self {
        Self::Express
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingStatus {
    Canceled,
    Delivered,
    Failure,
    Pending,
    Returned,
    Shipped,
}

impl IssuingCardShippingStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Canceled => "canceled",
            Self::Delivered => "delivered",
            Self::Failure => "failure",
            Self::Pending => "pending",
            Self::Returned => "returned",
            Self::Shipped => "shipped",
        }
    }
}

impl AsRef<str> for IssuingCardShippingStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardShippingStatus {
    fn default() -> Self {
        Self::Canceled
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuingCardShippingType {
    Bulk,
    Individual,
}

impl IssuingCardShippingType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Bulk => "bulk",
            Self::Individual => "individual",
        }
    }
}

impl AsRef<str> for IssuingCardShippingType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for IssuingCardShippingType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for IssuingCardShippingType {
    fn default() -> Self {
        Self::Bulk
    }
}
