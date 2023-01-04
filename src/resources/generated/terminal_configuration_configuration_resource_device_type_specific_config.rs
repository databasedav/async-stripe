#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TerminalConfigurationConfigurationResourceDeviceTypeSpecificConfig {
    /// A File ID representing an image you would like displayed on the reader.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub splashscreen: Option<Vec<crate::generated::File>>,
}
