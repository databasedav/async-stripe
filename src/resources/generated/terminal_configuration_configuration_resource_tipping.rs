#[derive(Clone, Debug, Default, serde::Deserialize, serde::Serialize)]
pub struct TerminalConfigurationConfigurationResourceTipping {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub aud:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub cad:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub chf:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub czk:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub dkk:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub eur:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub gbp:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub hkd:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub myr:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nok:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub nzd:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sek:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub sgd:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub usd:
        Option<crate::generated::TerminalConfigurationConfigurationResourceCurrencySpecificConfig>,
}
