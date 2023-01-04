/// A Mandate is a record of the permission a customer has given you to debit their payment method.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct Mandate {
    pub customer_acceptance: crate::generated::CustomerAcceptance,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub multi_use: Option<crate::generated::MandateMultiUse>,

    /// ID of the payment method associated with this mandate.
    pub payment_method: Vec<crate::generated::PaymentMethod>,

    pub payment_method_details: crate::generated::MandatePaymentMethodDetails,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub single_use: Option<crate::generated::MandateSingleUse>,

    /// The status of the mandate, which indicates whether it can be used to initiate a payment.
    pub status: MandateStatus,

    /// The type of the mandate.
    #[serde(rename = "type")]
    pub type_: MandateType,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetMandatesMandateParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateStatus {
    Active,
    Inactive,
    Pending,
}

impl MandateStatus {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Active => "active",
            Self::Inactive => "inactive",
            Self::Pending => "pending",
        }
    }
}

impl AsRef<str> for MandateStatus {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for MandateStatus {
    fn default() -> Self {
        Self::Active
    }
}

#[derive(Clone, Copy, Debug, serde::Deserialize, Eq, PartialEq, serde::Serialize)]
#[serde(rename_all = "snake_case")]
pub enum MandateType {
    MultiUse,
    SingleUse,
}

impl MandateType {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::MultiUse => "multi_use",
            Self::SingleUse => "single_use",
        }
    }
}

impl AsRef<str> for MandateType {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl std::fmt::Display for MandateType {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        self.as_str().fmt(f)
    }
}

impl Default for MandateType {
    fn default() -> Self {
        Self::MultiUse
    }
}
pub fn get_mandates_mandate(
    client: &crate::Client,
    mandate: String,
    params: GetMandatesMandateParams,
) -> crate::Response<crate::generated::Mandate> {
    client.get_query(&format!("/mandates/{mandate}", mandate = mandate), params)
}
