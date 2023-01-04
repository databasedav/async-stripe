/// An early fraud warning indicates that the card issuer has notified us that a
/// charge may be fraudulent.
///
/// Related guide: [Early Fraud Warnings](https://stripe.com/docs/disputes/measuring#early-fraud-warnings).
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct RadarEarlyFraudWarning {
    /// An EFW is actionable if it has not received a dispute and has not been fully refunded.
    ///
    /// You may wish to proactively refund a charge that receives an EFW, in order to avoid receiving a dispute later.
    pub actionable: bool,

    /// ID of the charge this early fraud warning is for, optionally expanded.
    pub charge: Vec<crate::generated::Charge>,

    /// Time at which the object was created.
    ///
    /// Measured in seconds since the Unix epoch.
    pub created: crate::params::Timestamp,

    /// The type of fraud labelled by the issuer.
    ///
    /// One of `card_never_received`, `fraudulent_card_application`, `made_with_counterfeit_card`, `made_with_lost_card`, `made_with_stolen_card`, `misc`, `unauthorized_use_of_card`.
    pub fraud_type: String,

    /// Unique identifier for the object.
    pub id: String,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// ID of the Payment Intent this early fraud warning is for, optionally expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<Vec<crate::generated::PaymentIntent>>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetRadarEarlyFraudWarningsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub charge: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub payment_intent: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetRadarEarlyFraudWarningsEarlyFraudWarningParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

pub fn get_radar_early_fraud_warnings(
    client: &crate::Client,
    params: GetRadarEarlyFraudWarningsParams,
) -> crate::Response<crate::params::List<crate::generated::RadarEarlyFraudWarning>> {
    client.get_query("/radar/early_fraud_warnings", params)
}

pub fn get_radar_early_fraud_warnings_early_fraud_warning(
    client: &crate::Client,
    early_fraud_warning: String,
    params: GetRadarEarlyFraudWarningsEarlyFraudWarningParams,
) -> crate::Response<crate::generated::RadarEarlyFraudWarning> {
    client.get_query(
        &format!(
            "/radar/early_fraud_warnings/{early_fraud_warning}",
            early_fraud_warning = early_fraud_warning
        ),
        params,
    )
}
