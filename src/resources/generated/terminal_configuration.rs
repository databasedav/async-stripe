/// A Configurations object represents how features should be configured for terminal readers.
#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
pub struct TerminalConfiguration {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<
        crate::generated::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,

    /// Unique identifier for the object.
    pub id: String,

    /// Whether this Configuration is the default for your account.
    pub is_account_default: Option<bool>,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<crate::generated::TerminalConfigurationConfigurationResourceTipping>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<
        crate::generated::TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig,
    >,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParams {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<PostTerminalConfigurationsParamsBbposWiseposE>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<PostTerminalConfigurationsParamsTipping>,

    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<PostTerminalConfigurationsParamsVerifoneP400>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTerminalConfigurationsParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub ending_before: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_account_default: Option<bool>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub starting_after: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedGetTerminalConfigurationsConfiguration {
    TerminalConfiguration(crate::generated::TerminalConfiguration),
    DeletedTerminalConfiguration(crate::generated::DeletedTerminalConfiguration),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct GetTerminalConfigurationsConfigurationParams {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,
}

#[derive(Clone, Debug, serde::Deserialize, serde::Serialize)]
#[serde(untagged, rename_all = "snake_case")]
pub enum ReturnedPostTerminalConfigurationsConfiguration {
    TerminalConfiguration(crate::generated::TerminalConfiguration),
    DeletedTerminalConfiguration(crate::generated::DeletedTerminalConfiguration),
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParams {
    /// An object containing device type specific settings for BBPOS WisePOS E readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bbpos_wisepos_e: Option<PostTerminalConfigurationsConfigurationParamsBbposWiseposE>,

    /// Specifies which fields in the response should be expanded.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expand: Option<Vec<String>>,

    /// Tipping configurations for readers supporting on-reader tips.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tipping: Option<PostTerminalConfigurationsConfigurationParamsTipping>,

    /// An object containing device type specific settings for Verifone P400 readers.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verifone_p400: Option<PostTerminalConfigurationsConfigurationParamsVerifoneP400>,
}

/// An object containing device type specific settings for BBPOS WisePOS E readers.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsBbposWiseposE {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTipping {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<PostTerminalConfigurationsParamsTippingAud>,

    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<PostTerminalConfigurationsParamsTippingCad>,

    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<PostTerminalConfigurationsParamsTippingChf>,

    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<PostTerminalConfigurationsParamsTippingCzk>,

    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<PostTerminalConfigurationsParamsTippingDkk>,

    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<PostTerminalConfigurationsParamsTippingEur>,

    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<PostTerminalConfigurationsParamsTippingGbp>,

    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<PostTerminalConfigurationsParamsTippingHkd>,

    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<PostTerminalConfigurationsParamsTippingMyr>,

    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<PostTerminalConfigurationsParamsTippingNok>,

    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<PostTerminalConfigurationsParamsTippingNzd>,

    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<PostTerminalConfigurationsParamsTippingSek>,

    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<PostTerminalConfigurationsParamsTippingSgd>,

    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<PostTerminalConfigurationsParamsTippingUsd>,
}

/// An object containing device type specific settings for Verifone P400 readers.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsVerifoneP400 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsBbposWiseposE {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTipping {
    /// Tipping configuration for AUD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud: Option<PostTerminalConfigurationsConfigurationParamsTippingAud>,

    /// Tipping configuration for CAD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad: Option<PostTerminalConfigurationsConfigurationParamsTippingCad>,

    /// Tipping configuration for CHF.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf: Option<PostTerminalConfigurationsConfigurationParamsTippingChf>,

    /// Tipping configuration for CZK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk: Option<PostTerminalConfigurationsConfigurationParamsTippingCzk>,

    /// Tipping configuration for DKK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk: Option<PostTerminalConfigurationsConfigurationParamsTippingDkk>,

    /// Tipping configuration for EUR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur: Option<PostTerminalConfigurationsConfigurationParamsTippingEur>,

    /// Tipping configuration for GBP.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp: Option<PostTerminalConfigurationsConfigurationParamsTippingGbp>,

    /// Tipping configuration for HKD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd: Option<PostTerminalConfigurationsConfigurationParamsTippingHkd>,

    /// Tipping configuration for MYR.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr: Option<PostTerminalConfigurationsConfigurationParamsTippingMyr>,

    /// Tipping configuration for NOK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok: Option<PostTerminalConfigurationsConfigurationParamsTippingNok>,

    /// Tipping configuration for NZD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd: Option<PostTerminalConfigurationsConfigurationParamsTippingNzd>,

    /// Tipping configuration for SEK.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek: Option<PostTerminalConfigurationsConfigurationParamsTippingSek>,

    /// Tipping configuration for SGD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd: Option<PostTerminalConfigurationsConfigurationParamsTippingSgd>,

    /// Tipping configuration for USD.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd: Option<PostTerminalConfigurationsConfigurationParamsTippingUsd>,
}

#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsVerifoneP400 {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<String>,
}

/// Tipping configuration for AUD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingAud {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CAD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingCad {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CHF.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingChf {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CZK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingCzk {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for DKK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingDkk {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for EUR.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingEur {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for GBP.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingGbp {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for HKD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingHkd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for MYR.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingMyr {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for NOK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingNok {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for NZD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingNzd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for SEK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingSek {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for SGD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingSgd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for USD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsParamsTippingUsd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for AUD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingAud {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CAD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingCad {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CHF.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingChf {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for CZK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingCzk {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for DKK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingDkk {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for EUR.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingEur {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for GBP.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingGbp {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for HKD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingHkd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for MYR.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingMyr {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for NOK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingNok {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for NZD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingNzd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for SEK.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingSek {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for SGD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingSgd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

/// Tipping configuration for USD.
#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct PostTerminalConfigurationsConfigurationParamsTippingUsd {
    /// Fixed amounts displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub fixed_amounts: Option<Vec<i64>>,

    /// Percentages displayed when collecting a tip.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub percentages: Option<Vec<i64>>,

    /// Below this amount, fixed amounts will be displayed; above it, percentages will be displayed.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub smart_tip_threshold: Option<i64>,
}

pub fn post_terminal_configurations(
    client: &crate::Client,
    params: PostTerminalConfigurationsParams,
) -> crate::Response<crate::generated::TerminalConfiguration> {
    client.post_form("/terminal/configurations", params)
}

pub fn get_terminal_configurations(
    client: &crate::Client,
    params: GetTerminalConfigurationsParams,
) -> crate::Response<crate::params::List<crate::generated::TerminalConfiguration>> {
    client.get_query("/terminal/configurations", params)
}

pub fn get_terminal_configurations_configuration(
    client: &crate::Client,
    configuration: String,
    params: GetTerminalConfigurationsConfigurationParams,
) -> crate::Response<ReturnedGetTerminalConfigurationsConfiguration> {
    client.get_query(
        &format!("/terminal/configurations/{configuration}", configuration = configuration),
        params,
    )
}

pub fn post_terminal_configurations_configuration(
    client: &crate::Client,
    configuration: String,
    params: PostTerminalConfigurationsConfigurationParams,
) -> crate::Response<ReturnedPostTerminalConfigurationsConfiguration> {
    client.post_form(
        &format!("/terminal/configurations/{configuration}", configuration = configuration),
        params,
    )
}

pub fn delete_terminal_configurations_configuration(
    client: &crate::Client,
    configuration: String,
) -> crate::Response<crate::generated::DeletedTerminalConfiguration> {
    client
        .delete(&format!("/terminal/configurations/{configuration}", configuration = configuration))
}
